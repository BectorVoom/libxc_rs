//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 325/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk325(t1382: f64, t287: f64, t297: f64, t914: f64, t1378: f64, t312: f64, t894: f64, t1389: f64, t913: f64, t927: f64, t930: f64, t940: f64, t951: f64, t953: f64) -> (f64, f64, f64) {
    let t1396 = t287 * t1382;
    let t1397 = t1396 * t297;
    let t1398 = t914 * t1397;
    let t1401 = t914 * t1378;
    let t1404 = t312 * t1382;
    let t1405 = t1404 * t297;
    let t1406 = t894 * t1405;
    let t1411 = 0.11360101276506094136e1_f64 * t913 * t1398 + t927 + 0.28977204965962526182e-1_f64 * t930 * t1401 + 0.5848048239485271795e1_f64 * t940 * t1406 + t951 + 0.50380704458364197288e-2_f64 * t953 * t1389;
    (t1397, t1405, t1411)
}
