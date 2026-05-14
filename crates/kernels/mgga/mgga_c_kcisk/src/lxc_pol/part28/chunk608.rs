//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 608/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk608<F: Float>(t6905: F, t6940: F, t1689: F, t1809: F, t1860: F, t2399: F, t2505: F, t5084: F, t5085: F, t5087: F, t5089: F, t5134: F, t604: F, t6729: F, t6731: F, t6735: F, t6738: F, t674: F, t6741: F, t6743: F, t6747: F, t6791: F, t6884: F, t702: F) -> (F, F) {
    let t6941 = t6905 + t6940;
    let t6943 = t5084 + 0.23426533963880895498e-2 * t5085 + 0.46853067927761790996e-2 * t5087 + 0.23426533963880895498e-2 * t6729 + 0.46853067927761790996e-2 * t5089 * t6731 + 0.46853067927761790996e-2 * t1809 * t6735 + 0.46853067927761790996e-2 * t5134 * t6738 + 0.46853067927761790996e-2 * t6741 + 0.46853067927761790996e-2 * t1809 * t6743 + 0.14055920378328537299e-1 * t674 * t6747 - 0.46853067927761790996e-2 * t674 * t6791 - t6884 * t702 - t2399 * t1860 - t1689 * t2505 - t604 * t6941;
    (t6941, t6943)
}
