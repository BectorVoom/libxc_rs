//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1261/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1261(t10015: f64, t7749: f64, t3965: f64, t4479: f64, t6460: f64, t12475: f64, t6464: f64, t12143: f64, t7752: f64, t10027: f64, t3974: f64, t4475: f64, t6396: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22616 = 16.0_f64 / 15.0_f64 * t10015 * t7749;
    let t22619 = 16.0_f64 / 15.0_f64 * t3965 * t4479 * t6460;
    let t22622 = 32.0_f64 / 15.0_f64 * t12475 * t4479 * t6464;
    let t22624 = 16.0_f64 / 15.0_f64 * t12143 * t7752;
    let t22626 = 16.0_f64 / 15.0_f64 * t10027 * t7752;
    let t22629 = 16.0_f64 / 15.0_f64 * t3974 * t4475 * t6396;
    (t22616, t22619, t22622, t22624, t22626, t22629)
}
