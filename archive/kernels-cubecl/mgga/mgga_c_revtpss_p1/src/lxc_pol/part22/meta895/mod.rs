//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta895 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3086;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta895<F: Float>(t247: F, t42792: F, t4757: F, t4837: F, t15850: F, t3111: F, t3091: F, t43240: F, t4782: F, t41296: F, t42471: F, t11977: F, t4820: F, t1011: F, t4886: F, t697: F, t1065: F, t372: F, t4866: F, t11670: F, t15904: F, t12167: F, t11922: F, t16081: F, t16083: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t53431, t53433, t53437, t53473, t53479) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3086::<F>(t247, t42792, t4757, t4837, t15850, t3111, t3091, t43240, t4782, t41296, t42471, t11977, t4820);
        let (t53542, t53545, t53552, t53553, t53557) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3087::<F>(t1011, t4886, t697, t1065, t372, t4866, t11670, t15904, t12167, t11922, t16081, t16083);
    (t53431, t53433, t53437, t53473, t53479, t53542, t53545, t53552, t53553, t53557)
}
