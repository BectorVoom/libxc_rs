//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1026/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1026<F: Float>(t21429: F, t21479: F, t225: F, t68: F, t369: F, t14211: F, t17712: F, t4582: F, t21126: F, t977: F, t21122: F, t2979: F) -> (F, F, F, F, F, F, F, F) {
    let t21480 = t21429 + t21479;
    let t21481 = t21480 * t225;
    let t21482 = t21481 * t68;
    let t21483 = t21482 * t369;
    let t21486 = t17712 * t14211;
    let t21487 = t4582 * t21486;
    let t21490 = t977 * t21126;
    let t21493 = t2979 * t21122;
    (t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493)
}
