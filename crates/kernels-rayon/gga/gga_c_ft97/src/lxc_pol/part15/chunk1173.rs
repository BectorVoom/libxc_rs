//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1173/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1173(t4965: f64, t5299: f64, t10409: f64, t446: f64, t5225: f64, t43350: f64, t4969: f64, t2665: f64, t1212: f64, t21355: f64, t1193: f64, t21373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t89877 = t4965 * t5299;
    let t89879 = t446 * t10409 * t89877;
    let t89881 = t4965 * t5225;
    let t89883 = t446 * t43350 * t89881;
    let t89885 = t4969 * t5299;
    let t89887 = t446 * t2665 * t89885;
    let t89889 = t21355 * t1212;
    let t89891 = t446 * t10409 * t89889;
    let t89893 = t1193 * t21373;
    (t89877, t89879, t89881, t89883, t89885, t89887, t89889, t89891, t89893)
}
