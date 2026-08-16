//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 775/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk775(t1979: f64, t5484: f64, t5498: f64, t730: f64, t1987: f64, t1991: f64, t213: f64, t222: f64, t1862: f64, t667: f64, t1861: f64, t1867: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5500 = t5498 * t5484 * t1979;
    let t5502 = 0.10389515463408878255e3_f64 * t730 * t5500;
    let t5504 = 0.35089341735807877242e1_f64 * t1987 * t1991;
    let t5511 = 1.0_f64 / t213 / t222 / 4.0_f64;
    let t5512 = t1862 * t667;
    let t5513 = t5511 * t5512;
    let t5515 = t1861 * t667;
    let t5516 = t5515 * t1867;
    (t5500, t5502, t5504, t5511, t5512, t5513, t5516)
}
