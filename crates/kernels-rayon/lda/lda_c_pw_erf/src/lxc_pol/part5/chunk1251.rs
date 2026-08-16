//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1251/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1251(t19: f64, t644: f64, t647: f64, t7337: f64, t18280: f64, t18292: f64, t12874: f64, t2532: f64, t4763: f64, t6954: f64, t14005: f64, t20179: f64, t22403: f64, t22405: f64, t22407: f64, t22411: f64, t22412: f64, t22418: f64, t247: f64, t251: f64, t256: f64) -> (f64, f64, f64, f64, f64) {
    let t22422 = t7337 * t19 * t644 * t647;
    let t22424 = 4.0_f64 / 45.0_f64 * t18280;
    let t22425 = 16.0_f64 / 15.0_f64 * t18292;
    let t22427 = 8.0_f64 / 5.0_f64 * t12874 * t2532;
    let t22429 = 8.0_f64 / 5.0_f64 * t4763 * t6954;
    let t22430 = -t22403 - t22405 - t22407 + t22411 + t14005 - t22412 + t20179 * t247 * t251 * t256 / 3.0_f64 + t22418 / 3.0_f64 + 0.06077777777777778_f64 * t22422 - t22424 + t22425 - t22427 - t22429;
    (t22424, t22425, t22427, t22429, t22430)
}
