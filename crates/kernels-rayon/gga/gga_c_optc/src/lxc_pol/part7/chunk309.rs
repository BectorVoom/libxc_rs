//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 309/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk309(t115: f64, t852: f64, t5: f64, t363: f64, t362: f64, t857: f64, t357: f64, t355: f64, t176: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t987 = t852 * t115;
    let t988 = t987 * t5;
    let t989 = t988 * t363;
    let t992 = t857 * t362;
    let t993 = t357 * t992;
    let t995 = t355 * t993 / 6.0_f64;
    let t996 = t176 * t352;
    (t987, t988, t989, t992, t993, t995, t996)
}
