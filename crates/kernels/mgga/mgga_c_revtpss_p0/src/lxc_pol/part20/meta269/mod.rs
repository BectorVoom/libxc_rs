//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta269<F: Float>(t11687: F, t4894: F, t3117: F, t4900: F, t2258: F, t3094: F, t3093: F, t3092: F, t11644: F, t11649: F, t11653: F, t11656: F, t11663: F, t11667: F, t11672: F, t11675: F, t11680: F, t11684: F, t3091: F, t3097: F, t3130: F, t3136: F, t3169: F, t4837: F, t4892: F, t4899: F) -> (F, F, F, F, F, F, F, F) {
        let (t11688, t11689, t11692, t11693, t11696, t11697, t11698, t11701) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1118::<F>(t11687, t4894, t3117, t4900, t2258, t3094, t3093, t3092, t11644, t11649, t11653, t11656, t11663, t11667, t11672, t11675, t11680, t11684, t3091, t3097, t3130, t3136, t3169, t4837, t4892, t4899);
    (t11688, t11689, t11692, t11693, t11696, t11697, t11698, t11701)
}
