//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 756/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk756(t1397: f64, t9301: f64, t30208: f64, t493: f64, t1339: f64, t29969: f64, t1406: f64, t6575: f64, t29984: f64, t1381: f64, t3141: f64, t10215: f64, t203: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31182 = t1397 * t9301;
    let t31300 = t493 * t30208;
    let t31308 = t1339 * t29969;
    let t31356 = t1406 * t6575;
    let t31399 = t1339 * t29984;
    let t31428 = t3141 * t1381;
    let t31501 = t203 * t10215;
    (t31182, t31300, t31308, t31356, t31399, t31428, t31501)
}
