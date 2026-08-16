//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1503/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1503(t2204: f64, t6951: f64, t31700: f64, t575: f64, t31737: f64, t571: f64, t1913: f64, t8433: f64, t1921: f64, t8416: f64, t118108: f64, t118110: f64, t118203: f64, t2212: f64, t22533: f64, t31464: f64, t5790: f64, t6937: f64, t8331: f64, t8349: f64) -> f64 {
    let t118982 = t2204 * t6951;
    let t118983 = t31700 * t575;
    let t118984 = t571 * t31737;
    let t118985 = t1913 * t8433;
    let t118988 = t8416 * t1921;
    let t118990 = 2.0_f64 * t1921 * t31464 + t2212 * t22533 + 2.0_f64 * t5790 * t8433 + t6937 * t8349 + t6951 * t8331 + t118108 + t118110 + t118203 + t118982 + t118983 + t118984 + 2.0_f64 * t118985 + 2.0_f64 * t118988;
    t118990
}
