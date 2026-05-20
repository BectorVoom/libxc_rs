//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta910 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3113;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta910<F: Float>(t15731: F, t3169: F, t15816: F, t3168: F, t11710: F, t15591: F, t3091: F, t16060: F, t3241: F, t1011: F, t140: F, t16122: F, t12078: F, t53740: F, t12047: F, t16138: F, t372: F, t16158: F, t3106: F, t12003: F, t1659: F, t11648: F, t4879: F, t1063: F, t15790: F, t3172: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54733, t54739, t54785, t54792, t54795) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3113::<F>(t15731, t3169, t15816, t3168, t11710, t15591, t3091, t16060, t3241, t1011, t140, t16122);
        let (t54801, t54811, t54818, t54836, t54838, t54841, t54849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3114::<F>(t12078, t53740, t12047, t16138, t372, t16158, t3106, t12003, t1659, t11648, t4879, t1063, t15790, t3172);
    (t54733, t54739, t54785, t54792, t54795, t54801, t54811, t54818, t54836, t54838, t54841, t54849)
}
