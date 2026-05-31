//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1365/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1365<F: Float>(t17931: F, t432: F, t6584: F, t4844: F, t831: F, t161: F, t489: F, t6730: F, t6227: F, t166: F, t2599: F, t9321: F) -> (F, F, F, F, F, F) {
    let t17932 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t17931;
    let t17934 = t432 * t6584 / F::cast_from(15.0_f64);
    let t17935 = t831 * t4844;
    let t17936 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t17935;
    let t17938 = t161 * t489 * t6730;
    let t17939 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t17938;
    let t17941 = t432 * t6227 / F::cast_from(15.0_f64);
    let t17945 = t161 * t166 * t9321 * t2599 / F::cast_from(15.0_f64);
    (t17932, t17934, t17936, t17939, t17941, t17945)
}
