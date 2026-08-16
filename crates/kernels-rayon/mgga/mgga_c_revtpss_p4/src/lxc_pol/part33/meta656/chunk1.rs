//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2110/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2110(t114: f64, t101454: f64, t101456: f64, t101754: f64, t105870: f64, t105873: f64, t105876: f64, t105878: f64, t105881: f64, t105883: f64, t94974: f64, t94976: f64, t508: f64, t651: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t105885 = -t94974 - 11.0_f64 / 9.0_f64 * t94976 - t101754 - t101454 + t101456 - 2.0_f64 / 3.0_f64 * t105870 - 3.0_f64 / 4.0_f64 * t105873 + t105876 / 2.0_f64 + t105878 / 3.0_f64 + t105881 / 4.0_f64 - t105883 / 8.0_f64;
    let t105886 = piecewise3(t115, 0.0_f64, t105885);
    let t105889 = 2.0_f64 * t651 * t508 * t105886;
    (t105886, t105889)
}
