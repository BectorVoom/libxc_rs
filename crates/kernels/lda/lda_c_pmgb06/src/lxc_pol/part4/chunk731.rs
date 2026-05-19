//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 731/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk731<F: Float>(t453: F, t4672: F, t36: F, t136: F, t443: F, t138: F, t2897: F, t3081: F, t3082: F, t3084: F, t3086: F, t3088: F, t4635: F, t4637: F, t4640: F, t4642: F, t4647: F, t4652: F, t4657: F, t4661: F, t4665: F, t4670: F) -> (F, F, F, F, F) {
    let t4673 = t453 * t4672;
    let t4674 = t36 * t4673;
    let t4676 = t136 * t443;
    let t4678 = t138 * t2897 * t4676;
    let t4680 = t3081 + F::cast_from(0.0016792592592592592_f64) * t3082 - F::cast_from(0.0004198148148148148_f64) * t3084 + F::cast_from(0.0012594444444444445_f64) * t3086 - F::cast_from(0.0006297222222222223_f64) * t3088 + F::cast_from(0.0008396296296296296_f64) * t4635 - F::cast_from(0.0008396296296296296_f64) * t4637 + t4640 - F::cast_from(0.01385388888888889_f64) * t4642 + F::cast_from(0.002099074074074074_f64) * t4647 - F::cast_from(0.007556666666666666_f64) * t4652 + F::cast_from(0.005037777777777778_f64) * t4657 + F::cast_from(0.0012594444444444445_f64) * t4661 + F::new(0.011335) * t4665 - F::cast_from(0.015113333333333333_f64) * t4670 - F::cast_from(0.003778333333333333_f64) * t4674 + F::cast_from(0.003778333333333333_f64) * t4678;
    (t4673, t4674, t4676, t4678, t4680)
}
