//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1309;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta360<F: Float>(t10073: F, t10934: F, t253: F, t39552: F, t2783: F, t9646: F, t22: F, t251: F, t837: F, t2722: F, t860: F, t231: F, t2782: F, t10665: F, t2723: F, t4503: F, t10638: F, t10111: F, t2789: F, t588: F, t870: F, t10963: F, t9303: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39694, t39697, t39701, t39704, t39707) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1309::<F>(t10073, t10934, t253, t39552, t2783, t9646, t22, t251, t837, t2722, t860, t231, t2782);
        let (t39709, t39712, t39714, t39719, t39723, t39724) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1310::<F>(t10665, t251, t2723, t2782, t4503, t10638, t10111, t22, t2789, t588, t870, t10963, t9303);
    (t39694, t39697, t39701, t39704, t39707, t39709, t39712, t39714, t39719, t39723, t39724)
}
