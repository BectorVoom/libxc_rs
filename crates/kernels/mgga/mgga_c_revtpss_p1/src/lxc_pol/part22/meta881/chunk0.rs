//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3053/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3053<F: Float>(t22: F, t231: F, t39698: F, t4494: F, t2782: F, t2783: F, t51375: F, t10073: F, t14509: F, t10069: F, t40921: F, t4496: F) -> (F, F, F, F, F) {
    let t51676 = t39698 * t4494 * t231 * t22;
    let t51680 = t2782 * t2783 * t51375 * t231;
    let t51682 = t10073 * t14509;
    let t51684 = t10069 * t14509;
    let t51686 = t40921 * t4496;
    (t51676, t51680, t51682, t51684, t51686)
}
