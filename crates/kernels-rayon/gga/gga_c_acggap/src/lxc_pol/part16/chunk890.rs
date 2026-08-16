//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 890/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk890(t1530: f64, t7336: f64, t174: f64, t30423: f64, t3126: f64, t3157: f64, t7323: f64, t577: f64, t7851: f64, t339: f64, t1165: f64, t30327: f64, t3355: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t30698 = t1530 * t7336;
    let t30714 = t30423 * t7323 * t174 * t3157 * t3126;
    let t30715 = 0.12734375e-1_f64 * t30714;
    let t30716 = t7851 * t577;
    let t30717 = t30716 * t339;
    let t30725 = t30327 * t1165 * t604 * t3355;
    (t30698, t30715, t30716, t30717, t30725)
}
