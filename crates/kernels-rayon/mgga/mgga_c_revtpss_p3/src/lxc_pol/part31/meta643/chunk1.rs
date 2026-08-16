//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2102/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2102(t5915: f64, t665: f64, t25826: f64, t21876: f64, t6998: f64, t101454: f64, t101456: f64, t101754: f64, t105870: f64, t105873: f64, t105876: f64, t105878: f64, t94974: f64, t94976: f64) -> f64 {
    let t105880 = t5915 * t665;
    let t105881 = t25826 * t105880;
    let t105883 = t6998 * t21876;
    let t105885 = -t94974 - 11.0_f64 / 9.0_f64 * t94976 - t101754 - t101454 + t101456 - 2.0_f64 / 3.0_f64 * t105870 - 3.0_f64 / 4.0_f64 * t105873 + t105876 / 2.0_f64 + t105878 / 3.0_f64 + t105881 / 4.0_f64 - t105883 / 8.0_f64;
    t105885
}
