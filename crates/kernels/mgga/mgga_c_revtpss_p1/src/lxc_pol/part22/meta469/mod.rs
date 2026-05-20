//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2157;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2158;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2159;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta469<F: Float>(t15670: F, t366: F, t3106: F, t4817: F, t1025: F, t1028: F, t11644: F, t11649: F, t11783: F, t15651: F, t15656: F, t15662: F, t15668: F, t1665: F, t3208: F, t3211: F, t3220: F, t3224: F, t4854: F, t4858: F, t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15671, t15675, t15676) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2157::<F>(t15670, t366, t3106, t4817, t1025, t1028, t11644, t11649, t11783, t15651, t15656, t15662, t15668, t1665, t3208, t3211, t3220, t3224, t4854, t4858);
        let (t15682, t15684, t15687, t15688) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2158::<F>(t11710, t4787, t3091, t245, t4890, t3088);
        let t15689 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2159::<F>(t15688, t3317);
        let (t15690, t15691) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2160::<F>(t1065, t1668, t372);
    (t15671, t15675, t15676, t15682, t15684, t15687, t15688, t15689, t15690, t15691)
}
