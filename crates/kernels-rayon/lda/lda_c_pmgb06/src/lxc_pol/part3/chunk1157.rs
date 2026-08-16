//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1157/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1157(t4602: f64, t5474: f64, t176: f64, t1826: f64, t3194: f64, t493: f64, t10316: f64, t10319: f64, t10321: f64, t10333: f64, t13799: f64, t13801: f64, t13803: f64, t13806: f64, t13808: f64, t13810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13812 = 4.0_f64 / 9.0_f64 * t4602 * t5474;
    let t13816 = 2.0_f64 / 15.0_f64 * t493 * t3194 * t176 * t1826;
    let t13817 = 4.0_f64 / 135.0_f64 * t10316;
    let t13818 = 4.0_f64 / 135.0_f64 * t10319;
    let t13819 = 8.0_f64 / 405.0_f64 * t10321;
    let t13820 = 2.0_f64 / 15.0_f64 * t10333;
    let t13821 = t13799 - t13801 + t13803 + t13806 + t13808 + t13810 - t13812 - t13816 - t13817 - t13818 + t13819 + t13820;
    (t13812, t13816, t13817, t13818, t13819, t13820, t13821)
}
