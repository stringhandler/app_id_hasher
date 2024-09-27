use anyhow::Error;
use blake2::{Blake2b, Digest};
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
use std::io;

use blake2::digest::Update;
use blake2::digest::VariableOutput;
use blake2::Blake2bVar;
use tari_utilities::encoding::Base58;

fn main() {
    if let Err(e) = main_inner() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn main_inner() -> Result<(), anyhow::Error> {
    // Path to the input and output CSV files
    let input_file = "app_mining.csv";
    let output_file = "output.csv";

    // Open the input CSV file
    let file = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Open the output CSV file
    let out_file = File::create(output_file)?;
    let mut wtr = WriterBuilder::new().from_writer(out_file);

    // Get the headers from the input file
    let headers = rdr.headers()?.clone();

    // Write the headers along with the new "Hash" column
    let mut new_headers = headers.clone();
    new_headers.push_field("Hash");
    wtr.write_record(&new_headers)?;

    // Iterate over records in the CSV file
    for result in rdr.records() {
        let record = result?;
        let mut new_record = record.clone();

        // Select the column to hash, here it's column 0 (change as needed)
        let column_to_hash = record.get(1).unwrap();

        // Compute the 20-byte Blake2b hash of the selected column
        let mut hasher = Blake2bVar::new(20).unwrap();
        hasher.update(column_to_hash.as_bytes());
        let mut buf = [0u8; 20];
        hasher.finalize_variable(&mut buf).unwrap();

        // Convert the hash to a hex string
        // let hash_hex = hex::encode(hash_result);

        // Add the hash to the new record
        new_record.push_field(&buf.to_base58());

        // Write the new record to the output CSV file
        wtr.write_record(&new_record)?;
    }

    // Flush the writer to ensure all data is written
    wtr.flush()?;
    Ok(())
}
