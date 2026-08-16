//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 614/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk614(t170: f64, t3457: f64, t1602: f64, t529: f64, t166: f64, t161: f64, t3320: f64, t3324: f64, t3327: f64, t3328: f64, t3331: f64, t3335: f64, t3386: f64, t3387: f64, t3391: f64, t3392: f64, t3395: f64, t3445: f64, t3449: f64, t3452: f64, t3455: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3458 = t170 * t3457;
    let t3459 = t1602 * t529;
    let t3460 = t3458 * t3459;
    let t3461 = t166 * t3460;
    let t3463 = t161 * t3461 / 5.0_f64;
    let t3464 = 0.03354522822333102_f64 * t3320 + t3324 + t3327 + 0.21642082724729686_f64 * t3328 + t3331 - t3335 - t3386 + 4.0_f64 * t3387 + t3391 + 8.0_f64 * t3392 + t3395 - t3445 - t3449 + t3452 - t3455 - t3463;
    (t3458, t3459, t3460, t3461, t3463, t3464)
}
