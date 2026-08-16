//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 673/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk673(t208: f64, t395: f64, t206: f64, t1730: f64, t573: f64, t580: f64, t3287: f64, t3289: f64, t3294: f64, t3297: f64, t3299: f64, t3302: f64, t3305: f64, t3307: f64, t3386: f64, t3445: f64, t3449: f64, t3452: f64, t3455: f64, t3463: f64) -> (f64, f64, f64, f64, f64) {
    let t4159 = t395 * t208;
    let t4161 = 0.06649088888888889_f64 * t206 * t4159;
    let t4162 = t573 * t1730;
    let t4165 = 0.09973633333333333_f64 * t580 * t1730;
    let t4166 = t3287 + t3289 + t3294 + t3297 + t3299 + t3302 + t3305 - t3307 - t3386 - t3445 - t4161 + 0.09973633333333333_f64 * t4162 + t4165 - t3449 + t3452 - t3455 - t3463;
    (t4159, t4161, t4162, t4165, t4166)
}
