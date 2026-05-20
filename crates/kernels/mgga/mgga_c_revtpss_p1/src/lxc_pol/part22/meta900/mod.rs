//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta900 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3093;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta900<F: Float>(t11710: F, t16089: F, t16090: F, t11883: F, t4924: F, t1086: F, t15654: F, t3090: F, t11922: F, t16077: F, t3115: F, t225: F, t53222: F, t366: F, t1025: F, t371: F, t4852: F, t676: F, t53014: F, t11656: F, t15734: F, t11670: F, t370: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t53820, t53832, t53855, t53859, t53865) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3093::<F>(t11710, t16089, t16090, t11883, t4924, t1086, t15654, t3090, t11922, t16077, t3115, t225, t53222);
        let (t53866, t53875, t53877, t53881, t53884) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3094::<F>(t366, t53865, t1025, t371, t4852, t676, t225, t53014, t11656, t15734, t11670, t370);
    (t53820, t53832, t53855, t53859, t53865, t53866, t53875, t53877, t53881, t53884)
}
