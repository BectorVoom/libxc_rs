//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2401/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2401(t231: f64, t2782: f64, t2783: f64, t40888: f64, t2723: f64, t39704: f64, t4503: f64, t123: f64, t212: f64, t9291: f64, t2786: f64, t10073: f64, t10654: f64) -> (f64, f64, f64, f64, f64) {
    let t40914 = t2782 * t2783 * t40888 * t231;
    let t40918 = t2782 * t4503 * t39704 * t2723;
    let t40921 = t123 * t9291 * t212;
    let t40922 = t40921 * t2786;
    let t40924 = t10073 * t10654;
    (t40914, t40918, t40921, t40922, t40924)
}
