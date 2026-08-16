//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 930/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk930(t203: f64, t5700: f64, t674: f64, t11399: f64, t11398: f64, t3663: f64, t561: f64, t3665: f64, t1453: f64, t3673: f64, t169: f64, t8951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11401 = t5700 * t674 * t203;
    let t11402 = t11399 * t11401;
    let t11403 = t11398 * t11402;
    let t11405 = t561 * t3663;
    let t11406 = t11405 * t3665;
    let t11408 = t3673 * t1453;
    let t11409 = t169 * t11408;
    let t11410 = t11409 * t8951;
    (t11401, t11402, t11403, t11405, t11406, t11408, t11409, t11410)
}
