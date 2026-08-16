//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1027/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1027(t122: f64, t227: f64, t8088: f64, t107: f64, t1126: f64, t1180: f64, t199: f64, t2778: f64, t4182: f64, t610: f64, t1669: f64, t1735: f64) -> (f64, f64, f64, f64, f64) {
    let t10472 = 0.9079060239445599_f64 * t122 * t8088 * t227;
    let t10474 = t107 * t1180 * t1126;
    let t10479 = 2.0103076928521055_f64 * t2778 * t199;
    let t10487 = t122 * t4182 * t610;
    let t10490 = t122 * t1669 * t1735;
    (t10472, t10474, t10479, t10487, t10490)
}
