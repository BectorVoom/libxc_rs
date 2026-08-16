//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1074/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1074(t18965: f64, t11469: f64, t8438: f64, t18973: f64, t18976: f64, t18981: f64, t11466: f64, t11472: f64, t14423: f64, t19882: f64, t8414: f64, t8417: f64, t8419: f64, t8423: f64, t8427: f64, t8432: f64, t8437: f64, t8445: f64, t8449: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20081 = 0.032530742648344574_f64 * t18965;
    let t20082 = 36.0_f64 * t11469;
    let t20084 = 3.5089340384731225_f64 * t8438;
    let t20085 = 3.5089340384731225_f64 * t18973;
    let t20086 = 0.0007324622014701264_f64 * t18976;
    let t20087 = 0.0005493466511025948_f64 * t18981;
    let t20088 = t11466 + t8414 + t8417 + t8419 + t20081 - t20082 + t11472 + t8423 - t8427 - 1.898172889849454_f64 * t19882 + t8432 + t8437 - t20084 + t8445 - t8449 - t14423 + t20085 + t20086 - t20087;
    (t20081, t20082, t20084, t20085, t20086, t20087, t20088)
}
