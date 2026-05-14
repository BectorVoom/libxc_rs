//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 689/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk689<F: Float>(t36: F, t4664: F, t1: F, t1531: F, t332: F, t453: F, t1830: F, t1074: F, t1863: F, t136: F, t443: F, t138: F, t2897: F, t3081: F, t3082: F, t3084: F, t3086: F, t3088: F, t4635: F, t4637: F, t4640: F, t4642: F, t4647: F, t4652: F, t4657: F, t4661: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4665 = t36 * t4664;
    let t4667 = t1531 * t1;
    let t4668 = t4667 * t332;
    let t4669 = t453 * t4668;
    let t4670 = t1830 * t4669;
    let t4672 = t1863 * t1074;
    let t4673 = t453 * t4672;
    let t4674 = t36 * t4673;
    let t4676 = t136 * t443;
    let t4678 = t138 * t2897 * t4676;
    let t4680 = t3081 + 0.0016792592592592592 * t3082 - 0.0004198148148148148 * t3084 + 0.0012594444444444445 * t3086 - 0.0006297222222222223 * t3088 + 0.0008396296296296296 * t4635 - 0.0008396296296296296 * t4637 + t4640 - 0.01385388888888889 * t4642 + 0.002099074074074074 * t4647 - 0.007556666666666666 * t4652 + 0.005037777777777778 * t4657 + 0.0012594444444444445 * t4661 + 0.011335 * t4665 - 0.015113333333333333 * t4670 - 0.003778333333333333 * t4674 + 0.003778333333333333 * t4678;
    (t4665, t4668, t4669, t4670, t4672, t4673, t4674, t4676, t4678, t4680)
}
