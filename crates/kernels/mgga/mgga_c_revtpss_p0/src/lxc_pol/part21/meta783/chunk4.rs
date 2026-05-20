//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2813/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2813<F: Float>(t231: F, t2782: F, t2783: F, t51306: F, t22: F, t39698: F, t4494: F, t51375: F, t10073: F, t14509: F, t10069: F, t40921: F, t4496: F) -> (F, F, F, F, F, F) {
    let t51672 = t2782 * t2783 * t51306 * t231;
    let t51676 = t39698 * t4494 * t231 * t22;
    let t51680 = t2782 * t2783 * t51375 * t231;
    let t51682 = t10073 * t14509;
    let t51683 = F::cast_from(0.19514881078765566038e-2_f64) * t51682;
    let t51684 = t10069 * t14509;
    let t51685 = F::cast_from(0.21951497276451705329e-1_f64) * t51684;
    let t51686 = t40921 * t4496;
    (t51672, t51676, t51680, t51683, t51685, t51686)
}
