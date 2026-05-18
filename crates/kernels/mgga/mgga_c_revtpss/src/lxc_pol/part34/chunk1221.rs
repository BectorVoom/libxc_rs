//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1221/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1221<F: Float>(t25895: F, t98028: F, t1892: F, t7063: F, t25877: F, t26069: F, t97922: F, t10073: F, t25937: F, t7282: F, t7910: F, t25899: F, t97899: F) -> (F, F, F, F, F, F) {
    let t98029 = t25895 * t98028;
    let t98040 = t7063 * t1892;
    let t98041 = t98040 * t25877;
    let t98084 = t26069 * t97922;
    let t98099 = t10073 * t7282 * t25937 * t7910;
    let t98101 = t25899 * t97899;
    (t98029, t98040, t98041, t98084, t98099, t98101)
}
