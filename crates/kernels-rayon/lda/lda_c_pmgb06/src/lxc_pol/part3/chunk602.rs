//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 602/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk602(t3306: f64, t186: f64, t409: f64, t55: f64, t543: f64, t1400: f64, t27: f64, t545: f64, t1403: f64, t3271: f64, t3273: f64, t3275: f64, t3278: f64, t3282: f64, t3287: f64, t3289: f64, t3294: f64, t3297: f64, t3299: f64, t3302: f64, t3305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3307 = t3306 / 45.0_f64;
    let t3309 = t55 * t409 * t186;
    let t3311 = 0.09618703433213194_f64 * t543 * t3309;
    let t3312 = t1400 * t27;
    let t3313 = t3312 * t545;
    let t3315 = t1403 * t27;
    let t3316 = t3315 * t545;
    let t3318 = t3271 + t3273 + t3275 + t3278 + t3282 + t3287 + t3289 + t3294 + t3297 + t3299 + t3302 + t3305 - t3307 - t3311 + 0.3246312408709453_f64 * t3313 + 0.6492624817418906_f64 * t3316;
    (t3307, t3309, t3311, t3312, t3313, t3315, t3316, t3318)
}
