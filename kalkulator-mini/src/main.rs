use std::io;

const NAMA_APP: &str = "Kalkulator Mini";
const VERSI: &str = "1.0";

fn main() {
    println!("=== {NAMA_APP} v{VERSI} ===");

    let mut jalan: bool = true;

    while jalan {
        println!("----------");
        println!("\nMenu:");
        println!("1. Tambah");
        println!("2. Kurang");
        println!("3. Kali");
        println!("4. Bagi");
        println!("5. Keluar");

        print!("Pilih (1-5): ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).unwrap();
        let pilihan: u32 = match pilihan.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input bukan angka!");
                continue;
            }
        };

        if pilihan == 5 {
            println!("Terima kasih!");
            jalan = false;
            continue;
        }

        if pilihan < 1 || pilihan > 4 {
            println!("Pilihan tidak valid.");
            continue;
        }

        // Input angka pertama
        print!("Angka pertama: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a: f64 = match a.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Bukan angka!");
                continue;
            }
        };

        // Input angka kedua
        print!("Angka kedua: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();
        let b: f64 = match b.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Bukan angka!");
                continue;
            }
        };

        // Hitung dengan if/else
        if pilihan == 1 {
            println!("Hasil: {}", a + b);
        } else if pilihan == 2 {
            println!("Hasil: {}", a - b);
        } else if pilihan == 3 {
            println!("Hasil: {}", a * b);
        } else if pilihan == 4 {
            if b == 0.0 {
                println!("Error: tidak bisa dibagi nol!");
            } else {
                println!("Hasil: {}", a / b);
            }
        }
    }
}
