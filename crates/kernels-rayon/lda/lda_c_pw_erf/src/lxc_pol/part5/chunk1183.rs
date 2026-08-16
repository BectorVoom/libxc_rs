//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1183/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1183(t3794: f64, t7589: f64, t1325: f64, t1440: f64, t15975: f64, t806: f64, t2098: f64, t6979: f64, t1472: f64, t7558: f64, t4804: f64, t21489: f64, t21494: f64, t21496: f64, t21498: f64, t21500: f64, t21505: f64, t21509: f64, t21513: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21515 = 4.0_f64 / 5.0_f64 * t3794 * t7589;
    let t21519 = 4.0_f64 / 5.0_f64 * t1325 * t1440 * t15975 * t806;
    let t21523 = 4.0_f64 / 5.0_f64 * t1325 * t1440 * t6979 * t2098;
    let t21525 = 4.0_f64 / 5.0_f64 * t1472 * t7558;
    let t21527 = 4.0_f64 / 5.0_f64 * t4804 * t7589;
    let t21528 = -t21489 + t21494 + t21496 - t21498 - t21500 - t21505 + t21509 + t21513 - t21515 - t21519 - t21523 + t21525 - t21527;
    (t21515, t21519, t21523, t21525, t21527, t21528)
}
