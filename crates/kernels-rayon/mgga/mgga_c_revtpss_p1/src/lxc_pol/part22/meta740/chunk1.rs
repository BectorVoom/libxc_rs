//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2805/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2805(t798: f64, t9726: f64, t802: f64, t10899: f64, t794: f64, t159: f64, t216: f64, t2475: f64, t123: f64, t212: f64, t9291: f64, t2786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40861 = t9726 * t798;
    let t40862 = t40861 * t802;
    let t40864 = t794 * t10899;
    let t40868 = t216 * t159 * t2475;
    let t40921 = t123 * t9291 * t212;
    let t40922 = t40921 * t2786;
    (t40861, t40862, t40864, t40868, t40921, t40922)
}
