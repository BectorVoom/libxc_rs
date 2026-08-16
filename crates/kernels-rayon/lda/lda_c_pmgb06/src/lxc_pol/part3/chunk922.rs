//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 922/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk922(t1321: f64, t374: f64, t4044: f64, t73: f64, t3559: f64, t1227: f64, t384: f64, t1234: f64, t4232: f64, t1322: f64, t4233: f64, t123: f64, t317: f64, t3974: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10547 = t1321 * t1321;
    let t10548 = 1.0_f64 / t10547;
    let t10551 = t73 * t4044 * t374;
    let t10558 = t73 * t3559;
    let t10565 = t384 * t1227;
    let t10570 = t4232 * t1234 * t374;
    let t10577 = t1322 * t384;
    let t10578 = t10577 * t4233;
    let t10582 = t4232 * t1227 * t374;
    let t10594 = t123 * t740 * t3974 * t317;
    (t10548, t10551, t10558, t10565, t10570, t10577, t10578, t10582, t10594)
}
