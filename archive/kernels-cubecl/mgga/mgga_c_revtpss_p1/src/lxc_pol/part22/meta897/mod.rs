//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta897 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta897<F: Float>(t11727: F, t4834: F, t16143: F, t3127: F, t3172: F, t15772: F, t3106: F, t15775: F, t15905: F, t43420: F, t43574: F, t11922: F, t15781: F, t4892: F) -> (F, F, F, F, F, F, F) {
        let (t53628, t53633, t53641, t53643, t53654, t53657, t53661) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3089::<F>(t11727, t4834, t16143, t3127, t3172, t15772, t3106, t15775, t15905, t43420, t43574, t11922, t15781, t4892);
    (t53628, t53633, t53641, t53643, t53654, t53657, t53661)
}
