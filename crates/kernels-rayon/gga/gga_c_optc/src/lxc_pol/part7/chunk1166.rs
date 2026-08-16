//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1166/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1166(t7724: f64, t778: f64, t800: f64, t2410: f64, t7624: f64, t2415: f64, t2449: f64, t2419: f64, t2378: f64, t7664: f64, t7668: f64, t774: f64) -> (f64, f64, f64, f64, f64) {
    let t24221 = t7724 * t778;
    let t24223 = 4.0_f64 * t24221 * t800;
    let t24225 = 6.0_f64 * t7624 * t2410;
    let t24226 = t2449 * t2415;
    let t24228 = 0.96490945932906628932e2_f64 * t24226 * t2419;
    let t24230 = 4.0_f64 * t2378 * t7664;
    let t24231 = t774 * t7668;
    (t24223, t24225, t24228, t24230, t24231)
}
