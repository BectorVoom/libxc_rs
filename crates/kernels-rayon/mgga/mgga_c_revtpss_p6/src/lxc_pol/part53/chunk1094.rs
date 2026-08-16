//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1094/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1094(t1032: f64, t786: f64, t119835: f64, t119893: f64, t39643: f64, t8476: f64, t31798: f64, t136: f64, t2457: f64, t8480: f64, t119822: f64, t25386: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119967 = t786 * t1032;
    let t119968 = t119967 * t119835;
    let t119969 = t119968 * t119893;
    let t119971 = t8476 * t39643;
    let t119972 = t119971 * t31798;
    let t119974 = t8480 * t136 * t2457;
    let t119976 = 0.6019057092162847523e-2_f64 * t119972 * t119974;
    let t119982 = t25386 * t119822;
    (t119967, t119968, t119969, t119971, t119974, t119976, t119982)
}
