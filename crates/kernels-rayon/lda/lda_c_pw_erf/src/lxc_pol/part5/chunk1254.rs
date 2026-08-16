//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1254/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1254(t331: f64, t7405: f64, t7409: f64, t10195: f64, t10202: f64, t10225: f64, t16287: f64, t16292: f64, t16297: f64, t16325: f64, t16327: f64, t16338: f64, t16345: f64, t16365: f64, t16370: f64, t16372: f64, t16374: f64, t16382: f64, t16397: f64, t16399: f64, t21847: f64, t25: f64, t589: f64) -> f64 {
    let t22484 = t331 * t7405;
    let t22486 = t331 * t7409;
    let t22498 = 0.08_f64 * t16287 - 0.14396666666666666_f64 * t16292 + 0.03999074074074074_f64 * t16297 - 0.07198333333333333_f64 * t16325 + 0.023994444444444443_f64 * t16327 - 0.09597777777777777_f64 * t16338 + 0.09597777777777777_f64 * t16345 + t10195 + 0.019753086419753086_f64 * t10202 + 0.0044444444444444444_f64 * t22484 + 0.0019753086419753087_f64 * t22486 - 0.006666666666666667_f64 * t25 * t589 * t21847 + t10225 + 0.044444444444444446_f64 * t16365 - 0.022222222222222223_f64 * t16370 - 0.007407407407407408_f64 * t16372 + 0.035991666666666665_f64 * t16374 + 0.013333333333333334_f64 * t16382 - 0.047988888888888886_f64 * t16397 - 0.03199259259259259_f64 * t16399;
    t22498
}
