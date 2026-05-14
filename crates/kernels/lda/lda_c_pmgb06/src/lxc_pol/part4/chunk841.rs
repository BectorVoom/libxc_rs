//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 841/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk841<F: Float>(t2584: F, t432: F, t3081: F, t3082: F, t4635: F, t4637: F, t4640: F, t4642: F, t6162: F, t6167: F, t6177: F, t6180: F, t6183: F, t6187: F, t6191: F, t6205: F, t6207: F, t6209: F, t6222: F) -> (F, F) {
    let t6657 = t432 * t2584 / 30.0;
    let t6673 = t3081 + 0.0008396296296296296 * t3082 + 0.0016792592592592592 * t4635 - 0.0008396296296296296 * t4637 + t4640 - 0.002518888888888889 * t4642 - 0.0004198148148148148 * t6205 + 0.002099074074074074 * t6180 - 0.007556666666666666 * t6177 + 0.005037777777777778 * t6183 + 0.0012594444444444445 * t6207 + 0.011335 * t6187 - 0.015113333333333333 * t6191 - 0.0006297222222222223 * t6209 + 0.0012594444444444445 * t6167 - 0.003778333333333333 * t6162 + 0.0018891666666666666 * t6222;
    (t6657, t6673)
}
