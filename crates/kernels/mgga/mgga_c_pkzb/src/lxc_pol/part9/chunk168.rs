//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 168/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk168<F: Float>(t513: F, t83: F, t106: F, t101: F, t477: F, t479: F, t483: F, t488: F) -> (F, F, F, F, F) {
    let t514 = t83 * t513;
    let t518 = t106 * t106;
    let t519 = 1.0 / t518;
    let t520 = t101 * t519;
    let t525 = -0.1176575e1 * t477 - 0.516475e0 * t479 - 0.2103875e0 * t483 - 0.104195e0 * t488;
    (t514, t518, t519, t520, t525)
}
