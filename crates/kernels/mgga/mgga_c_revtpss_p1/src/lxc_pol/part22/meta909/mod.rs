//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta909 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3111;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta909<F: Float>(t127: F, t15700: F, t15702: F, t4806: F, t16208: F, t372: F, t15666: F, t3211: F, t15656: F, t3215: F, t1025: F, t1663: F, t2434: F, t371: F, t15649: F, t225: F, t53166: F, t1053: F, t15655: F, t3224: F, t11991: F, t4817: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54667, t54672, t54678, t54680, t54687) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3111::<F>(t127, t15700, t15702, t4806, t16208, t372, t15666, t3211, t15656, t3215, t1025, t1663, t2434, t371);
        let (t54693, t54695, t54699, t54704, t54708) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3112::<F>(t1025, t127, t15649, t371, t225, t53166, t1053, t15655, t15666, t3224, t11991, t4817);
    (t54667, t54672, t54678, t54680, t54687, t54693, t54695, t54699, t54704, t54708)
}
