//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 545/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk545(t123: f64, t199: f64, t2281: f64, t125: f64, t1798: f64, t722: f64, t868: f64, t395: f64, t902: f64, t1155: f64, t1158: f64, t1161: f64, t1205: f64, t1206: f64, t1808: f64, t2164: f64, t305: f64, t566: f64, t726: f64, t81: f64, t912: f64) -> (f64, f64, f64, f64, f64) {
    let t2283 = t123 * t2281 * t199;
    let t2285 = t125 * t1798;
    let t2293 = t123 * t722 * t868;
    let t2302 = t395 * t902;
    let t2306 = -t1155 + 0.053059442957798957_f64 * t1158 + 0.053059442957798957_f64 * t1161 + 0.053059442957798957_f64 * t2283 - 0.031835665774679375_f64 * t123 * t2285 * t199 - 0.031835665774679375_f64 * t123 * t912 * t566 + 0.053059442957798957_f64 * t2293 - 0.031835665774679375_f64 * t123 * t726 * t868 - 0.031835665774679375_f64 * t123 * t305 * t1808 + t1205 - 0.10665013548435875_f64 * t1206 - 0.10665013548435875_f64 * t2302 + 0.05332506774217938_f64 * t81 * t2164;
    (t2283, t2285, t2293, t2302, t2306)
}
