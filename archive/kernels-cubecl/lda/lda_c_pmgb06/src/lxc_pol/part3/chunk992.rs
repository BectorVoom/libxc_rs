//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 992/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk992<F: Float>(t432: F, t4966: F, t132: F, t435: F, t4816: F, t486: F, t4941: F, t1730: F, t2025: F, t2021: F, t3209: F, t439: F, t6550: F) -> (F, F, F, F, F, F) {
    let t11790 = t432 * t4966 / F::cast_from(10.0_f64);
    let t11792 = t132 * t435 * t4816;
    let t11793 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t11792;
    let t11795 = t486 * t4941 / F::cast_from(5.0_f64);
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    let t11799 = F::cast_from(0.09973633333333333_f64) * t11798;
    let t11802 = t439 * t6550 * t3209 / F::cast_from(5.0_f64);
    (t11790, t11793, t11795, t11796, t11799, t11802)
}
