//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3033/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3033(t2475: f64, t808: f64, t14787: f64, t50768: f64, t14476: f64, t689: f64, t887: f64, t11028: f64, t1580: f64, t2439: f64, t10504: f64, t15002: f64, t9285: f64) -> (f64, f64, f64, f64, f64) {
    let t51176 = t808 * t2475;
    let t51178 = t50768 * t51176 * t14787;
    let t51196 = t689 * t14476 * t887;
    let t51199 = t2439 * t11028 * t1580;
    let t51203 = t10504 * t15002 * t9285;
    (t51176, t51178, t51196, t51199, t51203)
}
