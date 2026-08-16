//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 216/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk216(t1232: f64, t129: f64, t453: f64, t143: f64, t463: f64, t155: f64, t462: f64, t153: f64, t122: f64, t594: f64, t169: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1233 = t1232 * t129;
    let t1234 = t453 * t453;
    let t1238 = t143 * t143;
    let t1240 = 1.0_f64 / t1238 / t143;
    let t1242 = t1240 * pi * t463;
    let t1246 = 1.0_f64 / t462 / t155;
    let t1247 = t153 * t1246;
    let t1338 = t122 * t594;
    let t1339 = t169 * t599;
    (t1233, t1234, t1238, t1240, t1242, t1246, t1247, t1338, t1339)
}
