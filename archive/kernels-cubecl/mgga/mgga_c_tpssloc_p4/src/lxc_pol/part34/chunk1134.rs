//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1134/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1134<F: Float>(t22705: F, t28130: F, t81228: F, t22704: F, t28134: F, t80798: F, t22892: F, t22893: F, t28148: F, t22751: F, t28149: F, t28139: F) -> (F, F, F, F, F) {
    let t97043 = t81228 * t22705 * t28130;
    let t97049 = t22704 * t80798 * t28134;
    let t97070 = t22892 * t22893 * t28148;
    let t97095 = t22751 * t28149;
    let t97108 = t22751 * t28139;
    (t97043, t97049, t97070, t97095, t97108)
}
