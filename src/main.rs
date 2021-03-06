use std::env;
use std::io;
use std::io::Read;

const MAX_WIDTH: usize = 55;
const NEWLINE: &str = "%0D%0A";

fn parse_input_to_vector() -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    loop {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("failed to read from pipe");
        input = input.trim().to_string();

        if input == "" {
            break;
        }

        lines.extend(input_to_center_aligned_strings(&input));
    }
    lines
}

fn input_to_center_aligned_strings(input: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
 
    let input = input.replace("\n", &format!(" {} ", NEWLINE));
    let input = input.split(" ");

    let mut line = String::from("");
    for word in input {
        if word == NEWLINE {
            lines.push(center_align(&line));
            line = String::from("");
        } else if word.len() + line.len() <= MAX_WIDTH {
            line = line.to_owned() + " " + word;
        } else {
            lines.push(center_align(&line));
            line = String::from(word);
        }
    }
    if line.len() > 0 {
        lines.push(center_align(&line));
    }
    lines
}

fn center_align(line: &str) -> String {
    let mut index = 0;
    let mut x = line.clone().to_string();
    while x.len() <= MAX_WIDTH {
        if index % 2 == 0 {
            x.push_str(" ");
        } else {
            x = String::from(" ") + &x;
        }
        index += 1;
    }
    x
}

fn parse_arguments_to_vector() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Vec::<String>::new();
    } else {
        let input = args[1..].join(" ");
        return input_to_center_aligned_strings(&input);
    }
}

fn main() {
    println!("{}", GARF_TOP);

    let mut lines = parse_arguments_to_vector();
    if lines.len() == 0  {
        lines = parse_input_to_vector();
    }

    for line in &lines {
        println!("` m.. {} -m`", line);
    }

    println!("{}", GARF_BOTTOM);
}

const GARF_TOP: &str = "                ./sdNMNhooo/.          ./oo+smmdhs/`
              -hMMMMMmNMNs+:/oo-    -os/..+shdMMMMMNs`
             -NMMM//md`:smsos-`/s/-y+` -sshmyymy:oNNMo
             yMMsdd .dy  `/s.+s. :o` -s++h/`oy..hy`ymh
             dmMy.ms `+:   `- .y/  `so`o/. -/ `d+ :s:h
        so.  s+sm .s: .+ooo++oo+y+-y/+oo+++oo::: -h`o+  .sd:
       `:oys-.d`s-  :s+.`     `-sNNy/.`     `:s/ o` m`-oso//-
      -yhysos/sy`--s/`           ys`           /y- ohy++syhho-
     -so+++/:ydh/os`             s/             .y+yhoh:--.://
           /hhyoh+               s/              `yssy+d+//-  
        -/sdy::s+                s/               `h+y+-..:+y/
      /o/..-:+yy                 s/                .N:-.  - `oo                       
     :y`:` `-.N.                 s/                 oy   ` `.`m                       
     +h.``:. oy                  s/                 `N.:oy  `ys                       
     /Mo.:ds d:                  s/                  h/ :d/+yd`                       
    `h///om. N`                  s/                  ss  :d.`h-                       
    ym/` s+  N                   s/                  oy   so -h`                      
   /hdds-h   m`                  s/                  ss   .m`+hs                      
  `doys:s+   yo                  s/                 .m.    myhdh:                     
  /s+-` h/   .m:             `y- s/ `y-            .h:    `m/sy+d                     
  hs/   /s    -h+`           :Ms s/ /Mo           :h-     +s `-/h-                    
  ms:o` `y:    `+s/`         `y- s/ `y-         -sy.     -h`   ++o                    
 .dy-m:+ `s/`    `/so:.`      -+oso++/-`   `.-:os/      /y.  -.d/y                    
 :yy-m/+   yh:`     ./++o+++osh.     .yyo+++/:-`     `:yh`  +++h+d                    
 :yy:d``   ossy+-`          `-h+-...-+h:          `-+yoos   m:sh/h                    
 -ho/.     o+ ./syyo/:-....--:+hhhhymh+:-...--:+oymo:` +o   ::sd+s                    
 `m:o      +o     yo-:::/+N::-.` oo `-:+osddsso/:.N    s+    .-dy/
 `m:o      +s     y:     .m      oo       y+      N    y/    .-dy/
 `m:o      /s     h:     .m      oo       y+      N   `h-    .-dy/
 `m:o        do+-d/:``   .d      s+       h+ `    Ndasd      .-mm/
 `m..               sdfsf.d      s+       h+dfdsdf           .-mm/
 `m..                      /:::::yo::::::/                    .-m/
 `m..                                                         .-m/";

const GARF_BOTTOM: &str =
    "  m..                                                         `-m`                    
  o+y.``                                                     ../y                     
  .dy:+/o  :do+/:                                -N/+++m. ``oo+d.                     
   ssy:ys/ -y  .-:mo+++o+om             /+my+++//:N.   m` h+sdso                      
   `dsoo+- .h     m.     -d..----yo---..``m:      N   `N .shyoh`                      
    -hoo:. .h     N`     .d      s/       M:      N`  `m  +ysy.                       
     :yos  .m     N      :d      s/       M.      N. `:h  .y+`
      .ys` `yo+:.`N      :h      s/      `M-  ``.:mso+/-`+s.  
       `sh+.  `-/+sooo+/:+h.`````s/``..-:+Nsoo++/-`  `./do    
         .+oo:.``  `.:.::/+++ooooo++//:-.`  `:y+./oooo/.     
             +0oo                                ooo/ 
                ::oooe                        ssdfs:
                    garfsaygarfsaygarfsaygarfsay";
