//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1090/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1090(t1558: f64, t1563: f64, t17673: f64, t1820: f64, t1826: f64, t19994: f64, t19997: f64, t20007: f64, t20019: f64, t20027: f64, t348: f64, t352: f64, t406: f64, t408: f64, t5524: f64, t5527: f64, t5536: f64, t5539: f64, t5992: f64, t6005: f64, t6101: f64, t6111: f64, t7354: f64, t7360: f64, t7365: f64, t7370: f64, t8949: f64, t8962: f64, t943: f64) -> f64 {
    let t20283 = -28.0_f64 / 81.0_f64 * t8949 * t7354 * t348 + 8.0_f64 / 9.0_f64 * t6101 * t943 + 4.0_f64 / 9.0_f64 * t5524 * t19994 - 2.0_f64 / 3.0_f64 * t5527 * t19997 - t1820 * t5992 / 3.0_f64 - t1558 * t7360 * t348 / 9.0_f64 + t406 * t20007 / 3.0_f64 - 28.0_f64 / 81.0_f64 * t8962 * t7365 * t352 - 8.0_f64 / 9.0_f64 * t6111 * t943 + 4.0_f64 / 9.0_f64 * t5536 * t17673 + 2.0_f64 / 3.0_f64 * t5539 * t20019 - t1826 * t6005 / 3.0_f64 - t1563 * t7370 * t352 / 9.0_f64 + t408 * t20027 / 3.0_f64;
    t20283
}
