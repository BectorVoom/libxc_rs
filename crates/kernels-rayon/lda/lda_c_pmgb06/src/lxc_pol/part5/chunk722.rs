//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 722/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk722(t2584: f64, t432: f64, t3081: f64, t3082: f64, t4635: f64, t4637: f64, t4640: f64, t4642: f64, t6162: f64, t6167: f64, t6177: f64, t6180: f64, t6183: f64, t6187: f64, t6191: f64, t6205: f64, t6207: f64, t6209: f64, t6222: f64) -> (f64, f64) {
    let t6657 = t432 * t2584 / 30.0_f64;
    let t6673 = t3081 + 0.0008396296296296296_f64 * t3082 + 0.0016792592592592592_f64 * t4635 - 0.0008396296296296296_f64 * t4637 + t4640 - 0.002518888888888889_f64 * t4642 - 0.0004198148148148148_f64 * t6205 + 0.002099074074074074_f64 * t6180 - 0.007556666666666666_f64 * t6177 + 0.005037777777777778_f64 * t6183 + 0.0012594444444444445_f64 * t6207 + 0.011335_f64 * t6187 - 0.015113333333333333_f64 * t6191 - 0.0006297222222222223_f64 * t6209 + 0.0012594444444444445_f64 * t6167 - 0.003778333333333333_f64 * t6162 + 0.0018891666666666666_f64 * t6222;
    (t6657, t6673)
}
