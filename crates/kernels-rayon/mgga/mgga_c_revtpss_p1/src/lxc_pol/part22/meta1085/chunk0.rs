//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3937/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3937(t1456: f64, t1464: f64, t22533: f64, t22571: f64, t3: f64, t4154: f64, t47730: f64, t575: f64, t60607: f64, t60620: f64, t60624: f64, t60629: f64, t6951: f64, t75716: f64, t75720: f64, t75801: f64) -> f64 {
    let tv4rho42 = t3 * t575 * t75716 + 2.0_f64 * t1456 * t22571 + 2.0_f64 * t1464 * t22533 + t4154 * t6951 + 4.0_f64 * t47730 + 2.0_f64 * t60607 + 4.0_f64 * t60620 + 4.0_f64 * t60624 + 2.0_f64 * t60629 + 2.0_f64 * t75720 + t75801;
    tv4rho42
}
