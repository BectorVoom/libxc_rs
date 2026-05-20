//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta894 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3083;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3084;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta894<F: Float>(t1063: F, t15193: F, t247: F, t3109: F, t11710: F, t15600: F, t3091: F, t127: F, t4823: F, t11774: F, t3096: F, t11670: F, t15687: F, t3317: F, t15690: F, t15689: F, t15692: F, t11916: F, t15932: F, t11922: F, t11927: F, t16026: F, t15964: F, t11268: F, t4820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53363, t53389, t53391, t53393, t53401) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3083::<F>(t1063, t15193, t247, t3109, t11710, t15600, t3091, t127, t4823, t11774, t3096, t11670, t15687);
        let (t53402, t53405) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3084::<F>(t3317, t53401, t127, t15690);
        let (t53407, t53413, t53416, t53422, t53427) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3085::<F>(t15689, t15692, t53405, t11916, t15932, t11922, t11927, t16026, t11710, t15964, t3091, t11268, t4820);
    (t53363, t53389, t53391, t53393, t53401, t53402, t53405, t53407, t53413, t53416, t53422, t53427)
}
