//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1068/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1068<F: Float>(t1730: F, t2025: F, t2021: F, t1179: F, t4068: F, t871: F, t2029: F, t4119: F, t224: F, t4753: F, t1447: F, t5176: F) -> (F, F, F, F, F, F) {
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    let t11810 = t871 * t1179 * t4068;
    let t11813 = t2029 * t4119;
    let t11821 = t4753 * t224;
    let t11830 = t1447 * t5176;
    (t11796, t11798, t11810, t11813, t11821, t11830)
}
