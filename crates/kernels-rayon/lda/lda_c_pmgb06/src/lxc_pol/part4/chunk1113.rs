//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1113/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1113(t1423: f64, t5287: f64, t5226: f64, t5254: f64, t5211: f64, t5295: f64, t5248: f64, t5264: f64, t4619: f64, t464: f64, t1894: f64, t3213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13911 = t1423 * t5287;
    let t13913 = t1423 * t5226;
    let t13915 = t1423 * t5254;
    let t13917 = t5211 * t5295;
    let t13920 = t5211 * t5248;
    let t13922 = t5211 * t5264;
    let t13933 = t4619 * t464;
    let t13948 = t3213 * t1894;
    (t13911, t13913, t13915, t13917, t13920, t13922, t13933, t13948)
}
