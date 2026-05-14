//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1088/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1088<F: Float>(t1995: F, t7560: F, t1147: F, t1306: F, t17753: F, t20670: F, t20674: F, t20676: F, t20678: F, t20685: F, t20687: F, t20693: F, t20695: F, t2156: F, t6062: F, t7888: F, t803: F) -> (F, F) {
    let t20697 = 0.17544670867903938621e1 * t7560 * t1995;
    let t20698 = -6.0 * t1147 * t1306 * t17753 * t6062 - 3.0 * t1306 * t2156 * t7888 * t803 - t20670 - t20674 + t20676 + t20678 - t20685 - t20687 - t20693 - t20695 - t20697;
    (t20697, t20698)
}
