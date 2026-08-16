//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 944/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk944(t387: f64, t7088: f64, t384: f64, t783: f64, t2448: f64, t73: f64, t123: f64, t1316: f64, t2258: f64, t2308: f64, t2311: f64, t295: f64, t297: f64, t315: f64, t317: f64, t346: f64, t388: f64, t4027: f64, t4030: f64, t5590: f64, t5591: f64, t5593: f64, t5601: f64, t6104: f64, t6927: f64, t6951: f64, t6958: f64, t6961: f64, t787: f64) -> (f64, f64, f64, f64) {
    let t7089 = t7088 * t387;
    let t7099 = t384 * t783;
    let t7102 = t73 * t2448;
    let t7106 = (t6927 + t6951) * t295 + t5590 + 0.07982822673503073_f64 * t5591 - 0.10643763564670763_f64 * t5593 - 0.01197423401025461_f64 * t6958 - 0.01197423401025461_f64 * t297 * t6961 - t4027 + t346 * t2258 * t787 + 0.19816831758676853_f64 * t4030 + t346 * t7089 * t73 - t5601 + 0.020267214298646783_f64 * t123 * t315 * t6104 * t317 + 6.0_f64 * t1316 * t2258 * t2311 - t346 * t2308 * t7099 + 3.0_f64 * t1316 * t388 * t7102;
    (t7089, t7099, t7102, t7106)
}
