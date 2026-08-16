//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 312/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk312(t118: f64, t1266: f64, t61: f64, t119: f64, t482: f64, t101: f64, t132: f64, t433: f64, t472: f64, t78: f64, t423: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1267 = t1266 * t118;
    let t1268 = t61 * t1267;
    let t1273 = t482 * t119;
    let t1276 = t132 * t101;
    let t1277 = t1276 * t433;
    let t1280 = t1276 * t472;
    let t1283 = t78 * t101;
    let t1287 = t9 * t423;
    (t1268, t1273, t1277, t1280, t1283, t1287)
}
