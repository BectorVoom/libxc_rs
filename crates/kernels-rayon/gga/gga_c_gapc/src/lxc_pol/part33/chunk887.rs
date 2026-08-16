//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 887/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk887(t10293: f64, t6943: f64, t10292: f64, t10264: f64, t6179: f64, t6182: f64, t800: f64, t10229: f64, t6146: f64, t2536: f64, t10113: f64, t876: f64) -> (f64, f64, f64, f64, f64) {
    let t10294 = t10293 * t6943;
    let t10295 = t10292 * t10294;
    let t10297 = t10264 * t6179;
    let t10298 = t800 * t6182;
    let t10299 = t10297 * t10298;
    let t10301 = t10229 * t6146;
    let t10302 = t10293 * t2536;
    let t10303 = t10301 * t10302;
    let t10305 = t10113 * t876;
    (t10295, t10299, t10301, t10303, t10305)
}
