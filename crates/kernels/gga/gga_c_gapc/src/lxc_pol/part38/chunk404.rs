//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 404/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk404<F: Float>(t617: F, t683: F, t1552: F, t203: F, t618: F, t197: F, t1338: F, t201: F, t584: F, t604: F, t1717: F, t1726: F, t179: F, t192: F, t1921: F, t1924: F, t1929: F, t1933: F, t1937: F, t1941: F, t1945: F, t1948: F, t1953: F, t1955: F, t1958: F, t1960: F, t1965: F, t1970: F, t1972: F, t1976: F, t1980: F, t1985: F, t1988: F, t582: F, t613: F, t620: F, t629: F, t634: F, t649: F, t664: F, t669: F, t677: F) -> (F, F, F, F) {
    let t1991 = t617 * t683;
    let t1994 = t203 * t1552;
    let t1995 = t618 * t1994;
    let t1996 = t197 * t1995;
    let t2001 = t203 * t1338;
    let t2002 = t201 * t2001;
    let t2003 = t197 * t2002;
    let t2006 = t604 * t584;
    let t2009 = F::cast_from(0.19323635647535681158e-6_f64) * t1921 * t649 + F::cast_from(0.13900948042322754167e-2_f64) * t179 * t1924 + F::cast_from(0.40544431790108032986e-3_f64) * t613 * t1929 + F::cast_from(0.40544431790108032986e-3_f64) * t1933 * t620 - F::cast_from(0.12357942809624928455e-3_f64) * t1937 * t1941 - F::cast_from(0.41193142698749761516e-5_f64) * t1945 * t1948 - F::cast_from(0.33787026491756694155e-5_f64) * t1953 * t1955 - F::cast_from(0.12357942809624928455e-3_f64) * t1958 * t1960 + F::cast_from(0.687148483626368822e-6_f64) * t1726 * t1717 - F::cast_from(0.33816362383187442026e-5_f64) * t1965 * t677 - F::cast_from(0.96618178237678405792e-8_f64) * t1970 * t1972 + F::cast_from(0.21417029509352046616e-4_f64) * t669 * t1976 - F::cast_from(0.40544431790108032986e-3_f64) * t613 * t1980 - F::cast_from(0.13900948042322754167e-2_f64) * t179 * t1985 - F::cast_from(0.343574241813184411e-6_f64) * t1988 * t649 - F::cast_from(0.10821235962619981448e-3_f64) * t192 * t1991 - F::cast_from(0.11594181388521408695e-4_f64) * t192 * t1996 - F::cast_from(0.2318836277704281739e-4_f64) * t629 * t664 + F::cast_from(0.57970906942607043474e-5_f64) * t634 * t2003 + F::cast_from(0.6487109086417285278e-2_f64) * t582 * t2006;
    (t1991, t1996, t2003, t2009)
}
