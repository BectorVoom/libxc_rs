//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 271/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk271(t1276: f64, t82: f64, t73: f64, t75: f64, t80: f64, t295: f64, t299: f64, t1007: f64, t78: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1277 = t82 * t1276;
    let t1279 = t75 * t73;
    let t1281 = 132.0_f64 * t1279 * t80;
    let t1283 = 288.0_f64 * t295 * t299;
    let t1284 = t78 * t1007;
    let t1285 = 1.0_f64 / t1284;
    let t1287 = 156.0_f64 * t76 * t1285;
    let t1288 = -t1281 + t1283 - t1287;
    (t1277, t1279, t1281, t1285, t1287, t1288)
}
