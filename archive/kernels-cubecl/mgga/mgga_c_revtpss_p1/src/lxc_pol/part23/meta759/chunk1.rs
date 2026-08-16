//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2553/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2553<F: Float>(t15749: F, t3211: F, t16199: F, t372: F, t16208: F, t1025: F, t1663: F, t2434: F, t371: F, t225: F, t53166: F, t366: F) -> (F, F, F, F, F, F) {
    let t54648 = t3211 * t15749;
    let t54658 = t372 * t16199;
    let t54672 = t372 * t16208;
    let t54687 = t1025 * t371 * t2434 * t1663;
    let t54695 = t53166 * t225;
    let t54696 = t54695 * t366;
    (t54648, t54658, t54672, t54687, t54695, t54696)
}
