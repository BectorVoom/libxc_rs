//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2334;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta604<F: Float>(t10960: F, t2435: F, t2482: F, t39620: F, t686: F, t72: F, t879: F, t10073: F, t10934: F, t253: F, t39552: F, t2783: F, t9646: F, t22: F, t251: F, t837: F, t2722: F, t860: F, t231: F, t2782: F, t10665: F, t2723: F, t4503: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t39687, t39692, t39694, t39697, t39698) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2334::<F>(t10960, t2435, t2482, t39620, t686, t72, t879, t10073, t10934, t253, t39552, t2783, t9646);
        let (t39701, t39704, t39707, t39709, t39712) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2335::<F>(t22, t251, t39698, t837, t2722, t860, t231, t2782, t2783, t10665, t2723, t4503);
    (t39687, t39692, t39694, t39697, t39698, t39701, t39704, t39707, t39709, t39712)
}
