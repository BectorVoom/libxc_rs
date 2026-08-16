//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1190/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1190(t3850: f64, t3965: f64, t4479: f64, t4500: f64, t784: f64, t3403: f64, t3412: f64, t6762: f64, t10027: f64, t4476: f64, t3824: f64, t3974: f64, t4475: f64) -> (f64, f64, f64, f64, f64) {
    let t14013 = 8.0_f64 / 15.0_f64 * t3965 * t4479 * t3850;
    let t14014 = t4500 * t784;
    let t14017 = 8.0_f64 / 9.0_f64 * t3965 * t14014 * t3403;
    let t14020 = 16.0_f64 / 15.0_f64 * t3965 * t6762 * t3412;
    let t14022 = 16.0_f64 / 15.0_f64 * t10027 * t4476;
    let t14025 = 8.0_f64 / 15.0_f64 * t3974 * t4475 * t3824;
    (t14013, t14017, t14020, t14022, t14025)
}
