use image::{GenericImageView};
use colored::*;

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).expect("Erro ao abrir a imagem");
    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height);

    for y in (0..height).step_by((scale * 2) as usize) {
        for x in (0..width).step_by(scale as usize) {
            let pix = img.get_pixel(x, y);

            let block = "▓".truecolor(pix[0], pix[1], pix[2]);

            print!("{}", block);
        }
        println!();    }
}

fn main() {
    get_image("pug.png",4); 
    

}

