//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1039/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1039<F: Float>(t27971: F, t72: F, t5445: F, t79: F, t5392: F, t605: F, t5399: F, t1441: F, t1458: F, t5493: F, t88: F, t22473: F, t5464: F) -> (F, F, F, F, F, F, F) {
    let t27972 = t72 * t27971;
    let t27975 = t79 * t5445;
    let t27976 = t72 * t27975;
    let t27979 = t605 * t5392;
    let t27982 = t605 * t5399;
    let t28002 = t1441 * t1458;
    let t28007 = t88 * t5493;
    let t28012 = t22473 * t5464;
    (t27972, t27976, t27979, t27982, t28002, t28007, t28012)
}
