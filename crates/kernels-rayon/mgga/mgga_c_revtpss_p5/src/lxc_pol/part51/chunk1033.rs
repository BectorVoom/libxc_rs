//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1033/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1033(t119971: f64, t31798: f64, t136: f64, t2457: f64, t8480: f64, t119822: f64, t25386: f64, t119826: f64, t119830: f64, t32469: f64, t2670: f64, t31831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119972 = t119971 * t31798;
    let t119974 = t8480 * t136 * t2457;
    let t119976 = 0.6019057092162847523e-2_f64 * t119972 * t119974;
    let t119982 = t25386 * t119822;
    let t119983 = t119982 * t119826;
    let t119985 = t32469 * t119830;
    let t119989 = t31831 * t2670;
    (t119974, t119976, t119982, t119983, t119985, t119989)
}
