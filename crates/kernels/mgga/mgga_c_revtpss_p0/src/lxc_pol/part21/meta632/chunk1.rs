//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2401/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2401<F: Float>(t231: F, t2782: F, t2783: F, t40888: F, t2723: F, t39704: F, t4503: F, t123: F, t212: F, t9291: F, t2786: F, t10073: F, t10654: F) -> (F, F, F, F, F) {
    let t40914 = t2782 * t2783 * t40888 * t231;
    let t40918 = t2782 * t4503 * t39704 * t2723;
    let t40921 = t123 * t9291 * t212;
    let t40922 = t40921 * t2786;
    let t40924 = t10073 * t10654;
    (t40914, t40918, t40921, t40922, t40924)
}
