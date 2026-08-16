//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1107/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1107(t10164: f64, t10167: f64, t4647: f64, t515: f64, t3872: f64, t3974: f64, t4475: f64, t4489: f64, t784: f64, t3807: f64, t3965: f64, t3811: f64, t4479: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12949 = 16.0_f64 / 45.0_f64 * t10164;
    let t12950 = 32.0_f64 / 405.0_f64 * t10167;
    let t12951 = t4647 * t515;
    let t12952 = 4.0_f64 / 15.0_f64 * t12951;
    let t12955 = 16.0_f64 / 15.0_f64 * t3974 * t4475 * t3872;
    let t12956 = t4489 * t784;
    let t12959 = 16.0_f64 / 15.0_f64 * t3965 * t12956 * t3807;
    let t12962 = 16.0_f64 / 15.0_f64 * t3965 * t4479 * t3811;
    (t12949, t12950, t12952, t12955, t12959, t12962)
}
