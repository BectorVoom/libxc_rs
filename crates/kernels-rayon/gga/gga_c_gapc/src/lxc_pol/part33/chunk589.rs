//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 589/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk589(t3411: f64, t3415: f64, t1084: f64, t3127: f64, t2664: f64, t2660: f64, t3132: f64, t129: f64, t2520: f64, t1078: f64, t197: f64, t2493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3416 = t3411 * t3415;
    let t3418 = t1084 * t3127;
    let t3419 = t3418 * t2664;
    let t3421 = t2660 * t3132;
    let t3422 = t3421 * t2664;
    let t3424 = t2520 * t129;
    let t3425 = t3424 * t1078;
    let t3427 = t197 * t2493;
    (t3416, t3418, t3419, t3421, t3422, t3424, t3425, t3427)
}
