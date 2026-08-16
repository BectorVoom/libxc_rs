//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1167/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1167(t13933: f64, t2961: f64, t439: f64, t1972: f64, t2877: f64, t2876: f64, t493: f64, t5486: f64, t1444: f64, t5359: f64, t1380: f64, t2912: f64, t5280: f64) -> (f64, f64, f64, f64, f64) {
    let t13936 = t439 * t13933 * t2961 / 9.0_f64;
    let t13938 = 2.0_f64 / 15.0_f64 * t1972 * t2877;
    let t13941 = 2.0_f64 / 15.0_f64 * t493 * t5486 * t2876;
    let t13943 = 2.0_f64 / 15.0_f64 * t1444 * t5359;
    let t13947 = 2.0_f64 / 15.0_f64 * t493 * t1380 * t5280 * t2912;
    (t13936, t13938, t13941, t13943, t13947)
}
