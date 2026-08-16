//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1187/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1187(t2672: f64, t24567: f64, t24565: f64, t2748: f64, t7380: f64, t7448: f64, t946: f64, t24502: f64, t330: f64, t7453: f64, t7837: f64, t874: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24568 = t2672 * t2672;
    let t24569 = t24567 * t24568;
    let t24574 = t2748 * t24565;
    let t24575 = t24567 * t7380;
    let t24580 = t946 * t7448;
    let t24583 = t330 * t24502;
    let t24584 = t24583 * t7453;
    let t24594 = t874 * t888 * t7837;
    (t24569, t24574, t24575, t24580, t24584, t24594)
}
