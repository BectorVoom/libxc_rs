//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3034/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3034(t10505: f64, t137: f64, t15002: f64, t41011: f64, t11015: f64, t4325: f64, t4477: f64, t9292: f64, t14472: f64, t2439: f64, t887: f64, t14979: f64, t689: f64, t779: f64) -> (f64, f64, f64, f64, f64) {
    let t51207 = t41011 * t15002 * t137 * t10505;
    let t51211 = t4325 * t11015;
    let t51213 = t9292 * t4477;
    let t51216 = t2439 * t14472 * t887;
    let t51227 = t689 * t779 * t14979;
    (t51207, t51211, t51213, t51216, t51227)
}
