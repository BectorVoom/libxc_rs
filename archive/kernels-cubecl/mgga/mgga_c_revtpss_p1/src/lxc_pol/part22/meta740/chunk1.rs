//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2805/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2805<F: Float>(t798: F, t9726: F, t802: F, t10899: F, t794: F, t159: F, t216: F, t2475: F, t123: F, t212: F, t9291: F, t2786: F) -> (F, F, F, F, F, F) {
    let t40861 = t9726 * t798;
    let t40862 = t40861 * t802;
    let t40864 = t794 * t10899;
    let t40868 = t216 * t159 * t2475;
    let t40921 = t123 * t9291 * t212;
    let t40922 = t40921 * t2786;
    (t40861, t40862, t40864, t40868, t40921, t40922)
}
