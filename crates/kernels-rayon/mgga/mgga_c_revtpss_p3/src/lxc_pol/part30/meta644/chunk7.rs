//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2268/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2268(t29490: f64, t571: f64, t105792: f64, t105794: f64, t105796: f64, t105798: f64, t105800: f64, t105802: f64, t18217: f64, t2168: f64, t96684: f64, t96692: f64, t96694: f64, t97580: f64, t97586: f64) -> f64 {
    let t105804 = 2.0_f64 * t571 * t29490;
    let t105806 = t18217 * t2168 + t105792 + t105794 + t105796 + t105798 + t105800 + t105802 + t105804 + 2.0_f64 * t96684 + t96692 + t96694 + t97580 + t97586;
    t105806
}
