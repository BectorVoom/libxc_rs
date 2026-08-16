//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1161/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1161(t10102: f64, t11: f64, t13598: f64, t12160: f64, t1953: f64, t3633: f64, t10178: f64, t10195: f64, t10196: f64, t10202: f64, t13562: f64, t13564: f64, t13568: f64, t13571: f64, t13574: f64, t13577: f64, t13580: f64, t13583: f64, t13585: f64, t13587: f64, t13589: f64, t13592: f64, t13595: f64) -> (f64, f64, f64) {
    let t13600 = t11 * t10102 * t13598;
    let t13603 = t1953 * t3633 * t12160;
    let t13607 = 0.019753086419753086_f64 * t13562 + 0.28444444444444444_f64 * t13564 + 0.02666666666666667_f64 * t10178 + t10195 - 0.8638_f64 * t13568 + 0.8638_f64 * t13571 + 0.47988888888888886_f64 * t13574 - 0.8638_f64 * t13577 + 1.2957_f64 * t13580 - 0.10666666666666667_f64 * t13583 + 0.023994444444444443_f64 * t13585 + 0.03999074074074074_f64 * t13587 - 0.5278777777777778_f64 * t13589 - 0.023994444444444443_f64 * t13592 + 0.14396666666666666_f64 * t13595 - 0.10664197530864197_f64 * t13600 + 0.23994444444444443_f64 * t13603 - 0.008888888888888889_f64 * t10196 + 0.05925925925925926_f64 * t10202;
    (t13600, t13603, t13607)
}
