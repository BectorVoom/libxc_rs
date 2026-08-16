//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1147/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1147(t2695: f64, t374: f64, t4232: f64, t4359: f64, t7077: f64, t1322: f64, t7088: f64, t297: f64, t301: f64, t413: f64, t6716: f64, t1183: f64, t2414: f64) -> (f64, f64, f64, f64, f64) {
    let t15086 = t4232 * t2695 * t374;
    let t15089 = t4359 * t7077;
    let t15096 = t7088 * t1322;
    let t15102 = t297 * t6716 * t413 * t301;
    let t15106 = t297 * t2414 * t1183 * t301;
    (t15086, t15089, t15096, t15102, t15106)
}
