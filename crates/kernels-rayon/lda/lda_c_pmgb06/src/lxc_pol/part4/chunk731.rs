//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 731/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk731(t453: f64, t4672: f64, t36: f64, t136: f64, t443: f64, t138: f64, t2897: f64, t3081: f64, t3082: f64, t3084: f64, t3086: f64, t3088: f64, t4635: f64, t4637: f64, t4640: f64, t4642: f64, t4647: f64, t4652: f64, t4657: f64, t4661: f64, t4665: f64, t4670: f64) -> (f64, f64, f64, f64, f64) {
    let t4673 = t453 * t4672;
    let t4674 = t36 * t4673;
    let t4676 = t136 * t443;
    let t4678 = t138 * t2897 * t4676;
    let t4680 = t3081 + 0.0016792592592592592_f64 * t3082 - 0.0004198148148148148_f64 * t3084 + 0.0012594444444444445_f64 * t3086 - 0.0006297222222222223_f64 * t3088 + 0.0008396296296296296_f64 * t4635 - 0.0008396296296296296_f64 * t4637 + t4640 - 0.01385388888888889_f64 * t4642 + 0.002099074074074074_f64 * t4647 - 0.007556666666666666_f64 * t4652 + 0.005037777777777778_f64 * t4657 + 0.0012594444444444445_f64 * t4661 + 0.011335_f64 * t4665 - 0.015113333333333333_f64 * t4670 - 0.003778333333333333_f64 * t4674 + 0.003778333333333333_f64 * t4678;
    (t4673, t4674, t4676, t4678, t4680)
}
