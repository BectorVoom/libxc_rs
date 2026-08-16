//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 319/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk319(t1338: f64, t431: f64, t1249: f64, t159: f64, t104: f64, t405: f64, t14: f64, t445: f64, t73: f64, t348: f64, t108: f64, t19: f64) -> (f64, f64, f64, f64, f64) {
    let t1339 = t431 * t1338;
    let t1343 = t1249 * t159;
    let t1346 = t405 * t104;
    let t1347 = t1346 * t14;
    let t1352 = t73 * t445;
    let t1353 = t1352 * t348;
    let t1354 = t108 * t19;
    (t1339, t1343, t1347, t1353, t1354)
}
