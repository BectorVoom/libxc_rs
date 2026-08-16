//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 701/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk701(t1152: f64, t123: f64, t566: f64, t1166: f64, t315: f64, t199: f64, t1156: f64, t1200: f64, t722: f64, t290: f64, t642: f64, t247: f64, t701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4257 = t123 * t1152 * t566;
    let t4259 = t315 * t1166;
    let t4261 = t123 * t4259 * t199;
    let t4264 = t123 * t1156 * t566;
    let t4267 = t123 * t722 * t1200;
    let t4283 = 1.279801625812305_f64 * t642 * t290;
    let t4284 = t247 * t701;
    (t4257, t4259, t4261, t4264, t4267, t4283, t4284)
}
