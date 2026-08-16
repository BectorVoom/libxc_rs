//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1012/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1012<F: Float>(t11739: F, t1214: F, t248: F, t3509: F, t3570: F, t3506: F, t11159: F, t3440: F, t11168: F, t1177: F, t135: F, t3561: F) -> (F, F, F, F, F, F) {
    let t11741 = t248 * t1214 * t11739;
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11748 = t3440 * t11159;
    let t11751 = t1177 * t11168;
    let t11754 = t135 * t3561;
    (t11741, t11745, t11746, t11748, t11751, t11754)
}
