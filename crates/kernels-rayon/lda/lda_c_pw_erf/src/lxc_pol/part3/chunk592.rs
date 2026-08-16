//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 592/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk592(t2715: f64, t103: f64, t2: f64, t39: f64, t102: f64, t1568: f64, t427: f64, t10: f64, t127: f64, t3222: f64, t3251: f64, t3280: f64, t3282: f64, t3284: f64, t3288: f64, t3290: f64, t3291: f64, t3296: f64, t3302: f64, t3305: f64, t3313: f64, t3314: f64, t426: f64, t436: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64) {
    let t3318 = param_hyb_omega_0 * t2715;
    let t3319 = t103 * t2;
    let t3322 = 1.9486833333333333_f64 * t3318 * t3319 * t39;
    let t3325 = 17.53815_f64 * t102 * t427 * t1568;
    let t3326 = t3280 - t3282 - t3284 - t3288 - t3290 + 9.0_f64 / 2.0_f64 * t426 * t10 * t3291 - 29.3808_f64 * t127 * t3296 * t3222 - t3302 - t3305 - 1.46904_f64 * t127 * t436 * t3251 + t3313 + 17.62848_f64 * t127 * t3314 * t1568 - t3322 + t3325;
    (t3318, t3319, t3322, t3325, t3326)
}
