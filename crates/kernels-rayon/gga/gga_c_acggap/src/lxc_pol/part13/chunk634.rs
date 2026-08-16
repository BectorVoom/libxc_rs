//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 634/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk634(t3396: f64, t3403: f64, t3571: f64, t3574: f64, t3576: f64, t3622: f64, t3624: f64, t3634: f64, t3636: f64, t3638: f64, t3649: f64, t3653: f64, t3658: f64, t418: f64, t4901: f64, t4906: f64, t4910: f64, t4912: f64, t4918: f64, t4921: f64, t4926: f64, t4928: f64, t4932: f64) -> f64 {
    let t4945 = 0.80031500487063509014e-2_f64 * t4901 - t4906 + t4910 + 0.68598428988911579156e-2_f64 * t3396 * t4912 + t4918 - 0.42874018118069736972e-2_f64 * t3403 * t4921 - 0.42874018118069736972e-3_f64 * t4926 - 0.20007875121765877254e-2_f64 * t4928 + 0.34299214494455789578e-2_f64 * t418 * t4932 - 35.0_f64 / 108.0_f64 * t3571 - 35.0_f64 / 216.0_f64 * t3574 + 7.0_f64 / 72.0_f64 * t3576 - 7.0_f64 / 48.0_f64 * t3622 - 7.0_f64 / 144.0_f64 * t3624 + 7.0_f64 / 288.0_f64 * t3634 + 7.0_f64 / 144.0_f64 * t3636 + 7.0_f64 / 144.0_f64 * t3638 + t3649 + 0.17149607247227894789e-2_f64 * t3653 - 0.17149607247227894789e-2_f64 * t3658;
    t4945
}
