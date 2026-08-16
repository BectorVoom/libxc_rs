//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 616/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk616(t117: f64, t123: f64, t1650: f64, t315: f64, t1135: f64, t118: f64, t125: f64, t2825: f64, t2828: f64, t2831: f64, t2835: f64, t2840: f64, t2844: f64, t2846: f64, t2849: f64, t3467: f64, t3474: f64) -> (f64, f64, f64) {
    let t3478 = t123 * t315 * t1650 * t117;
    let t3481 = 0.1890324433388467_f64 * t1135 * t118;
    let t3482 = t2825 - 0.005926167098672845_f64 * t2828 - 0.01185233419734569_f64 * t2831 - 0.0014862827083471494_f64 * t2835 - t2840 - t2844 - t2846 + 0.01975389032890948_f64 * t2849 - 0.005388405304614574_f64 * t123 * t125 * t3467 * t117 - 0.07184540406152766_f64 * t3474 + 0.02694202652307287_f64 * t3478 + t3481;
    (t3478, t3481, t3482)
}
