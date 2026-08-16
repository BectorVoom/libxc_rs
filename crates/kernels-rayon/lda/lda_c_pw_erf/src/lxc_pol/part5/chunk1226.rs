//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1226/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1226(t17985: f64, t15824: f64, t3965: f64, t4479: f64, t5424: f64, t14014: f64, t5220: f64, t12968: f64, t2021: f64, t3974: f64, t4516: f64, t15727: f64, t4475: f64, t5305: f64) -> (f64, f64, f64, f64, f64) {
    let t22121 = 8.0_f64 / 15.0_f64 * t17985;
    let t22125 = 32.0_f64 / 15.0_f64 * t3965 * t4479 * t5424 * t15824;
    let t22129 = 16.0_f64 / 9.0_f64 * t3965 * t14014 * t5220 * t15824;
    let t22133 = 32.0_f64 / 15.0_f64 * t3974 * t12968 * t2021 * t4516;
    let t22137 = 32.0_f64 / 15.0_f64 * t3974 * t4475 * t5305 * t15727;
    (t22121, t22125, t22129, t22133, t22137)
}
