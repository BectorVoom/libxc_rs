//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta881 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3053;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta881<F: Float>(t22: F, t231: F, t39698: F, t4494: F, t2782: F, t2783: F, t51375: F, t10073: F, t14509: F, t10069: F, t40921: F, t4496: F, t14537: F, t10532: F, t14598: F, t50511: F, t2797: F, t1568: F, t2645: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51676, t51680, t51682, t51684, t51686) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3053::<F>(t22, t231, t39698, t4494, t2782, t2783, t51375, t10073, t14509, t10069, t40921, t4496);
        let (t51688, t51696, t51700, t51703, t51708) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3054::<F>(t10073, t14537, t10532, t14598, t231, t50511, t2782, t2797, t10069, t1568, t2645, t2783);
    (t51676, t51680, t51682, t51684, t51686, t51688, t51696, t51700, t51703, t51708)
}
