//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 862/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk862<F: Float>(t124: F, t6798: F, t2557: F, t46: F, t552: F, t1667: F, t2620: F, t568: F, t637: F, t4880: F, t2607: F, t496: F, t4883: F, t1009: F, t1542: F, t1545: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6800 = 0.19751673498613801407e-1 * t6798 * t124;
    let t6801 = t2557 * t46;
    let t6803 = 0.36622894612013090108e-3 * t6801 * t552;
    let t6804 = t2620 * t1667;
    let t6805 = 0.24415263074675393405e-3 * t6804;
    let t6806 = t637 * t568;
    let t6810 = 4.0 * t4880;
    let t6811 = t496 * t2607;
    let t6812 = 8.0 * t6811;
    let t6813 = 80.0 * t4883;
    let t6819 = t1542 * t1009;
    let t6820 = 20.0 * t6819;
    let t6821 = t1545 * t1009;
    (t6800, t6801, t6803, t6805, t6806, t6810, t6812, t6813, t6820, t6821)
}
