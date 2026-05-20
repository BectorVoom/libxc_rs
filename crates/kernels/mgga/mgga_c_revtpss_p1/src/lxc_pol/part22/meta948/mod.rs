//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta948 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3187;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta948<F: Float>(t12809: F, t12916: F, t17483: F, t12772: F, t17729: F, t17731: F, t3718: F, t44546: F, t5353: F, t45833: F, t58919: F, t127: F, t17693: F, t17695: F, t5302: F, t1261: F, t12879: F, t247: F, t5056: F, t12963: F, t5323: F, t225: F, t56587: F, t17795: F, t3172: F, t3711: F, t17759: F, t44425: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t59179, t59182, t59185, t59196, t59220) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3187::<F>(t12809, t12916, t17483, t12772, t17729, t17731, t3718, t44546, t5353, t45833, t58919, t127, t17693, t17695, t5302);
        let (t59233, t59239, t59241, t59269, t59320) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3188::<F>(t1261, t12879, t247, t5056, t12963, t5323, t225, t56587, t17795, t3172, t3711, t17729, t17759, t44425);
    (t59179, t59182, t59185, t59196, t59220, t59233, t59239, t59241, t59269, t59320)
}
