//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 570/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk570(t1072: f64, t1074: f64, t869: f64, t3272: f64, t1069: f64, t910: f64, t1084: f64, t3072: f64, t1087: f64, t3074: f64, t2636: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3314 = t869 * t1072 * t1074;
    let t3316 = t3272 * t1074;
    let t3318 = t1069 * t910;
    let t3320 = t1084 * t3072;
    let t3321 = t1087 * t3074;
    let t3322 = t2636 * t3321;
    (t3314, t3316, t3318, t3320, t3321, t3322)
}
