//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1156/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1156(t12064: f64, t4523: f64, t2137: f64, t4073: f64, t3445: f64, t822: f64, t2120: f64, t3387: f64, t2072: f64, t5045: f64, t2076: f64, t3390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13538 = t12064 * t4523;
    let t13539 = 16.0_f64 / 27.0_f64 * t13538;
    let t13540 = t4073 * t2137;
    let t13541 = 8.0_f64 / 15.0_f64 * t13540;
    let t13542 = t822 * t3445;
    let t13543 = 8.0_f64 / 15.0_f64 * t13542;
    let t13544 = t2120 * t3387;
    let t13545 = 8.0_f64 / 15.0_f64 * t13544;
    let t13547 = 8.0_f64 / 5.0_f64 * t5045 * t2072;
    let t13548 = t2076 * t3390;
    (t13539, t13541, t13543, t13545, t13547, t13548)
}
