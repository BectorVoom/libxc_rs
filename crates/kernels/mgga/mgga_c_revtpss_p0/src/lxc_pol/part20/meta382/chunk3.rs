//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1391/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1391<F: Float>(t2645: F, t860: F, t231: F, t2782: F, t2783: F, t39714: F, t251: F, t40321: F, t2723: F, t39704: F, t4503: F, t123: F, t212: F, t9291: F) -> (F, F, F, F, F, F) {
    let t40888 = t860 * t2645;
    let t40894 = t2782 * t2783 * t39714 * t231;
    let t40902 = t40321 * t251;
    let t40914 = t2782 * t2783 * t40888 * t231;
    let t40918 = t2782 * t4503 * t39704 * t2723;
    let t40921 = t123 * t9291 * t212;
    (t40888, t40894, t40902, t40914, t40918, t40921)
}
