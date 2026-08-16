//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 766/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk766(t1397: f64, t9301: f64, t30208: f64, t493: f64, t1339: f64, t29969: f64, t1406: f64, t6575: f64, t29984: f64, t1381: f64, t3141: f64, t2754: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31182 = t1397 * t9301;
    let t31300 = t493 * t30208;
    let t31308 = t1339 * t29969;
    let t31356 = t1406 * t6575;
    let t31399 = t1339 * t29984;
    let t31428 = t3141 * t1381;
    let t31585 = t2754 * t874;
    (t31182, t31300, t31308, t31356, t31399, t31428, t31585)
}
