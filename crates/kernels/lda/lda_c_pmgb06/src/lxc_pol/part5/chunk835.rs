//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 835/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk835<F: Float>(t4844: F, t486: F, t3005: F, t831: F, t1730: F, t2025: F, t2021: F, t1179: F, t4068: F, t871: F, t2029: F, t4119: F, t2007: F, t3213: F, t131: F, t1767: F) -> (F, F, F, F, F, F, F, F) {
    let t11757 = t486 * t4844;
    let t11758 = t11757 / 45.0;
    let t11777 = t831 * t3005;
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    let t11799 = 0.09973633333333333 * t11798;
    let t11810 = t871 * t1179 * t4068;
    let t11813 = t2029 * t4119;
    let t11860 = t3213 * t2007;
    let t11861 = 2.0 / 135.0 * t11860;
    let t11862 = t131 * t1767;
    (t11758, t11777, t11796, t11799, t11810, t11813, t11861, t11862)
}
