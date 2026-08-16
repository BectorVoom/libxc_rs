//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 899/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk899(t30714: f64, t577: f64, t7851: f64, t339: f64, t1181: f64, t16507: f64, t7351: f64, t7426: f64, t1165: f64, t30327: f64, t3355: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t30715 = 0.12734375e-1_f64 * t30714;
    let t30716 = t7851 * t577;
    let t30717 = t30716 * t339;
    let t30721 = t7426 * t1181 * t7351 * t16507;
    let t30725 = t30327 * t1165 * t604 * t3355;
    (t30715, t30716, t30717, t30721, t30725)
}
