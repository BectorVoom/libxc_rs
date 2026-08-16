//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1257/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1257(t13308: f64, t16527: f64, t5077: f64, t12677: f64, t493: f64, t5318: f64, t6119: f64, t486: f64, t6610: f64, t5115: f64, t802: f64, t16505: f64, t16507: f64, t16510: f64, t16512: f64, t16516: f64, t16518: f64, t16521: f64, t16523: f64, t16525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16530 = 16.0_f64 / 45.0_f64 * t5077 * t13308 * t16527;
    let t16531 = 4.0_f64 / 15.0_f64 * t12677;
    let t16534 = 2.0_f64 / 15.0_f64 * t493 * t6119 * t5318;
    let t16535 = t486 * t6610;
    let t16536 = 4.0_f64 / 45.0_f64 * t16535;
    let t16537 = t802 * t5115;
    let t16538 = 4.0_f64 / 45.0_f64 * t16537;
    let t16539 = -t16505 + t16507 - t16510 - t16512 - t16516 - t16518 - t16521 + t16523 + t16525 - t16530 - t16531 + t16534 - t16536 - t16538;
    (t16530, t16531, t16534, t16536, t16538, t16539)
}
