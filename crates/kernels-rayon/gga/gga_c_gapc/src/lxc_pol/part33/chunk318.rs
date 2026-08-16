//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 318/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk318(t131: f64, t20: f64, t1354: f64, t14: f64, t70: f64, t543: f64, t402: f64, t78: f64, t4: f64, t3: f64, t95: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1355 = t20 * t131;
    let t1356 = t1354 * t1355;
    let t1359 = t14 * t70;
    let t1360 = t543 * t1359;
    let t1361 = t78 * t402;
    let t1362 = t4 * t1361;
    let t1365 = t3 * t95;
    let t1366 = t545 * t1365;
    (t1355, t1356, t1360, t1362, t1365, t1366)
}
