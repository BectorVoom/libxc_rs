//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1113/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1113(t24131: f64, t1046: f64, t12519: f64, t3445: f64, t3488: f64, t17444: f64, t47377: f64, t5400: f64, t639: f64, t47766: f64, t7115: f64, t7505: f64) -> (f64, f64, f64, f64, f64) {
    let t47782 = 64.0_f64 / 405.0_f64 * t24131;
    let t47784 = 8.0_f64 / 15.0_f64 * t12519 * t1046;
    let t47786 = 4.0_f64 / 5.0_f64 * t3488 * t3445;
    let t47790 = 128.0_f64 / 27.0_f64 * t639 * t5400 * t17444 * t47377;
    let t47793 = 32.0_f64 / 15.0_f64 * t7115 * t7505 * t47766;
    (t47782, t47784, t47786, t47790, t47793)
}
