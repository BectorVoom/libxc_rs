//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta717 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta717<F: Float>(t10073: F, t10934: F, t253: F, t39552: F, t2783: F, t9646: F, t22: F, t251: F, t837: F, t10111: F, t2789: F, t588: F, t870: F) -> (F, F, F, F, F, F) {
        let (t39694, t39697, t39698, t39701, t39719, t39723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2752::<F>(t10073, t10934, t253, t39552, t2783, t9646, t22, t251, t837, t10111, t2789, t588, t870);
    (t39694, t39697, t39698, t39701, t39719, t39723)
}
