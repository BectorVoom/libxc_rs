//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 225/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk225(t512: f64, t537: f64, t541: f64, t546: f64, t560: f64, t593: f64, t595: f64, t600: f64, t605: f64, t120: f64, t138: f64, t124: f64, t616: f64) -> (f64, f64, f64) {
    let t637 = t512 + t537 + t541 - t546 + t560 + t593 + t595 - t600 - t605;
    let t641 = t120 * t138;
    let t642 = t124 * t616;
    (t637, t641, t642)
}
