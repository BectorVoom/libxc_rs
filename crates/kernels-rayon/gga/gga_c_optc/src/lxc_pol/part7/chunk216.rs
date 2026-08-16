//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 216/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk216(t40: f64, t592: f64, t558: f64, t85: f64, t1: f64, t60: f64, t508: f64, t518: f64, t84: f64, t75: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t593 = t40 * t592;
    let t595 = 0.19751789702565206229e-1_f64 * t558 * t85;
    let t596 = t60 * t1;
    let t598 = t518 * t508 * t84;
    let t600 = 0.18311555036753159941e-3_f64 * t596 * t598;
    let t601 = t60 * t75;
    (t593, t595, t596, t598, t600, t601)
}
