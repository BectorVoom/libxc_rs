//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 857/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk857(t197: f64, t7807: f64, t3336: f64, t333: f64, t474: f64, t2482: f64, t667: f64, t6851: f64, t4043: f64, t311: f64, t134: f64, t959: f64) -> (f64, f64, f64, f64) {
    let t9730 = t197 * t7807;
    let t9731 = t3336 * t9730;
    let t9733 = t474 * t333;
    let t9734 = t2482 * t9733;
    let t9736 = t6851 * t667;
    let t9737 = t9736 * t4043;
    let t9738 = t311 * t9737;
    let t9739 = t134 * t959;
    (t9731, t9734, t9738, t9739)
}
