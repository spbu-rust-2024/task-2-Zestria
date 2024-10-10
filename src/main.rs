use std::io;

fn main() {
    
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Вацаонима");

    let chars : Vec<char> = string.chars().collect();
    
    let mut the_best = 0;
    let mut maximum = 1;
    
    for i in 0..chars.len()-1 {
        println!("{}", i);
        for j in 0..min(i+1, chars.len()-i) {
            //println!("{} or {}\n", i-j, i+j);
            if chars[i-j] == chars[i+j] {
                if maximum < j*2 + 1 {
                    the_best = i;
                    maximum = j*2 + 1;
                }
            } else {
                break;
            }
        }

        for j in 0..min(i+1, chars.len()-i-1) {
            if chars[i-j] == chars[i+j+1] {
                if maximum < (j+1)*2 {
                    the_best = i;
                    maximum = (j+1)*2;
                }
            } else {
                break;
            }
            
        }
    }
    
    if maximum % 2 == 0 {
        for i in the_best+1-maximum/2..the_best+1+maximum/2 {
            print!("{}", chars[i]);
        }
    } else {
        for i in the_best-maximum/2..the_best+maximum/2+1 {
            print!("{}", chars[i]);
        }
    }
    print!("\n");
}


fn min (a : usize, b : usize) -> usize {
    if a < b { a }
    else { b }
}
