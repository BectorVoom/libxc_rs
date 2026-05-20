//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta902 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3097;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta902<F: Float>(t1011: F, t15154: F, t15993: F, t15130: F, t15135: F, t11821: F, t140: F, t15140: F, t11710: F, t15614: F, t3091: F, t1063: F, t15937: F, t3172: F, t11672: F, t15682: F, t12078: F, t53552: F, t16183: F, t73: F, t42793: F, t4892: F, t4895: F, t15951: F, t3127: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53964, t53967, t53970, t53972, t53974, t53993, t53998) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3097::<F>(t1011, t15154, t15993, t15130, t15135, t11821, t140, t15140, t11710, t15614, t3091, t1063, t15937, t3172);
        let (t54014, t54023, t54026, t54036, t54039) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3098::<F>(t11672, t15682, t12078, t53552, t16183, t73, t42793, t4892, t4895, t15951, t3127, t3172);
    (t53964, t53967, t53970, t53972, t53974, t53993, t53998, t54014, t54023, t54026, t54036, t54039)
}
