//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1179/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1179(t2193: f64, t9752: f64, t2171: f64, t3780: f64, t1480: f64, t5334: f64, t1488: f64, t4804: f64, t5378: f64, t5382: f64, t4753: f64, t4943: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13899 = 4.0_f64 / 5.0_f64 * t9752 * t2193;
    let t13901 = 4.0_f64 / 5.0_f64 * t2171 * t3780;
    let t13903 = 4.0_f64 / 15.0_f64 * t5334 * t1480;
    let t13905 = 4.0_f64 / 9.0_f64 * t5334 * t1488;
    let t13906 = t4804 * t5378;
    let t13907 = 16.0_f64 / 15.0_f64 * t13906;
    let t13909 = 4.0_f64 / 5.0_f64 * t4804 * t5382;
    let t13911 = 8.0_f64 / 5.0_f64 * t4753 * t4943;
    (t13899, t13901, t13903, t13905, t13907, t13909, t13911)
}
