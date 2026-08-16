//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 904/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk904(t163: f64, t169: f64, t299: f64, t4239: f64, t4026: f64, t568: f64, t185: f64, t3678: f64, t514: f64, t196: f64, t3674: f64, t211: f64, t3656: f64) -> (f64, f64, f64, f64, f64) {
    let t9215 = t169 * t299 * t4239 * t163;
    let t9217 = t4026 * t568;
    let t9220 = t185 * t514 * t3678;
    let t9223 = 1.0_f64 / t3674 / t196;
    let t9231 = t211 * t514 * t3656;
    (t9215, t9217, t9220, t9223, t9231)
}
