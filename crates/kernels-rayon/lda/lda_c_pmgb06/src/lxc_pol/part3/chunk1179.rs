//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1179/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1179(t13372: f64, t13343: f64, t13345: f64, t13347: f64, t13350: f64, t13353: f64, t13356: f64, t13359: f64, t13362: f64, t13365: f64, t13368: f64, t13370: f64, t13374: f64, t13376: f64, t13379: f64, t9938: f64, t9940: f64, t9954: f64, t9956: f64, t9958: f64) -> f64 {
    let t14127 = 0.03199259259259259_f64 * t13372;
    let t14136 = -0.8638_f64 * t13343 + 0.023994444444444443_f64 * t13345 + 0.03999074074074074_f64 * t13347 - 0.023994444444444443_f64 * t13350 - 0.10664197530864197_f64 * t13353 + 1.2957_f64 * t13356 + 0.14396666666666666_f64 * t13359 + 0.23994444444444443_f64 * t13362 + 0.07198333333333333_f64 * t13365 - 0.4319_f64 * t13368 + 0.09597777777777777_f64 * t13370 - t14127 - 0.07198333333333333_f64 * t13374 + 1.5836333333333332_f64 * t13376 - 0.14396666666666666_f64 * t13379 + 0.05925925925925926_f64 * t9938 + 0.02666666666666667_f64 * t9940 + 0.044444444444444446_f64 * t9954 - 0.022222222222222223_f64 * t9956 - 0.007407407407407408_f64 * t9958;
    t14136
}
