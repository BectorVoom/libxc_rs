//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 476/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk476(t115: f64, t2341: f64, t5: f64, t363: f64, t988: f64, t992: f64, t355: f64, t287: f64, t529: f64, t362: f64, t357: f64, t984: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2342 = t2341 * t115;
    let t2343 = t2342 * t5;
    let t2344 = t2343 * t363;
    let t2347 = t988 * t992;
    let t2348 = t355 * t2347;
    let t2350 = t529 * t287;
    let t2351 = t2350 * t362;
    let t2352 = t357 * t2351;
    let t2354 = t355 * t2352 / 9.0_f64;
    let t2357 = t984 * t993;
    (t2343, t2344, t2347, t2348, t2350, t2351, t2352, t2354, t2357)
}
