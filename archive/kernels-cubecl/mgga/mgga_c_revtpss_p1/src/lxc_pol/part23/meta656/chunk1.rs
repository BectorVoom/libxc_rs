//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2385/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2385<F: Float>(t159: F, t216: F, t2475: F, t123: F, t212: F, t9291: F, t2786: F, t10914: F, t2710: F, t9285: F, t2790: F, t9292: F) -> (F, F, F, F, F) {
    let t40868 = t216 * t159 * t2475;
    let t40921 = t123 * t9291 * t212;
    let t40922 = t40921 * t2786;
    let t40945 = t2710 * t10914 * t9285;
    let t40958 = t9292 * t2790;
    (t40868, t40921, t40922, t40945, t40958)
}
