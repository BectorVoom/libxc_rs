//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 783/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk783(t12527: f64, t21370: f64, t2464: f64, t2465: f64, t6914: f64, t9176: f64, t20884: f64, t30845: f64, t900: f64, t9086: f64, t9561: f64, t123: f64, t883: f64, t9127: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40178 = t21370 * t12527;
    let t40182 = t6914 * t2464 * t2465 * t9176;
    let t40184 = t30845 * t20884;
    let t40186 = t900 * t9086;
    let t40187 = t9561 * t40186;
    let t40190 = t9127 * t123 * t883;
    (t40178, t40182, t40184, t40186, t40187, t40190)
}
