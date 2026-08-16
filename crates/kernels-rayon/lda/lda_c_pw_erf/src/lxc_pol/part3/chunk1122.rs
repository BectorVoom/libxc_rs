//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1122/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1122(t1333: f64, t4507: f64, t1403: f64, t352: f64, t743: f64, t4506: f64, t10030: f64, t5157: f64, t5162: f64, t10166: f64, t1325: f64, t4753: f64, t5356: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13122 = t4507 * t1333;
    let t13124 = t743 * t1403 * t352;
    let t13127 = 16.0_f64 / 15.0_f64 * t4506 * t13122 * t13124;
    let t13128 = t10030 * t5157;
    let t13129 = 32.0_f64 / 45.0_f64 * t13128;
    let t13130 = t10030 * t5162;
    let t13131 = 64.0_f64 / 45.0_f64 * t13130;
    let t13133 = 8.0_f64 / 15.0_f64 * t1325 * t10166;
    let t13135 = 4.0_f64 / 5.0_f64 * t4753 * t5356;
    (t13124, t13127, t13129, t13131, t13133, t13135)
}
