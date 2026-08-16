//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 212/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk212(t580: f64, t75: f64, t520: f64, t522: f64, t526: f64, t531: f64) -> (f64, f64) {
    let t581 = t75 * t580;
    let t586 = -0.86308333333333333334e0_f64 * t520 - 0.301925e0_f64 * t522 - 0.5501625e-1_f64 * t526 - 0.82785e-1_f64 * t531;
    (t581, t586)
}
