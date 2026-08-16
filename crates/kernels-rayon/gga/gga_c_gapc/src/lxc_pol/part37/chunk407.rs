//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 407/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk407(t617: f64, t683: f64, t1552: f64, t203: f64, t618: f64, t197: f64, t1338: f64, t201: f64, t584: f64, t604: f64, t1717: f64, t1726: f64, t179: f64, t192: f64, t1921: f64, t1924: f64, t1929: f64, t1933: f64, t1937: f64, t1941: f64, t1945: f64, t1948: f64, t1953: f64, t1955: f64, t1958: f64, t1960: f64, t1965: f64, t1970: f64, t1972: f64, t1976: f64, t1980: f64, t1985: f64, t1988: f64, t582: f64, t613: f64, t620: f64, t629: f64, t634: f64, t649: f64, t664: f64, t669: f64, t677: f64) -> (f64, f64, f64, f64) {
    let t1991 = t617 * t683;
    let t1994 = t203 * t1552;
    let t1995 = t618 * t1994;
    let t1996 = t197 * t1995;
    let t2001 = t203 * t1338;
    let t2002 = t201 * t2001;
    let t2003 = t197 * t2002;
    let t2006 = t604 * t584;
    let t2009 = 0.19323635647535681158e-6_f64 * t1921 * t649 + 0.13900948042322754167e-2_f64 * t179 * t1924 + 0.40544431790108032986e-3_f64 * t613 * t1929 + 0.40544431790108032986e-3_f64 * t1933 * t620 - 0.12357942809624928455e-3_f64 * t1937 * t1941 - 0.41193142698749761516e-5_f64 * t1945 * t1948 - 0.33787026491756694155e-5_f64 * t1953 * t1955 - 0.12357942809624928455e-3_f64 * t1958 * t1960 + 0.687148483626368822e-6_f64 * t1726 * t1717 - 0.33816362383187442026e-5_f64 * t1965 * t677 - 0.96618178237678405792e-8_f64 * t1970 * t1972 + 0.21417029509352046616e-4_f64 * t669 * t1976 - 0.40544431790108032986e-3_f64 * t613 * t1980 - 0.13900948042322754167e-2_f64 * t179 * t1985 - 0.343574241813184411e-6_f64 * t1988 * t649 - 0.10821235962619981448e-3_f64 * t192 * t1991 - 0.11594181388521408695e-4_f64 * t192 * t1996 - 0.2318836277704281739e-4_f64 * t629 * t664 + 0.57970906942607043474e-5_f64 * t634 * t2003 + 0.6487109086417285278e-2_f64 * t582 * t2006;
    (t1991, t1996, t2003, t2009)
}
