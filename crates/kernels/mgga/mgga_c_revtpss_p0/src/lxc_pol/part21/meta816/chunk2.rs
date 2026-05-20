//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2997/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2997<F: Float>(t1025: F, t1663: F, t2434: F, t371: F, t127: F, t15649: F, t225: F, t53166: F, t366: F, t1053: F, t15655: F, t15666: F, t3224: F) -> (F, F, F, F, F, F) {
    let t54687 = t1025 * t371 * t2434 * t1663;
    let t54693 = t1025 * t371 * t127 * t15649;
    let t54695 = t53166 * t225;
    let t54696 = t54695 * t366;
    let t54699 = t15655 * t1053;
    let t54704 = t3224 * t15666;
    (t54687, t54693, t54695, t54696, t54699, t54704)
}
