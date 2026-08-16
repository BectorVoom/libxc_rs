//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 983/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk983(t43: f64, t11401: f64, t3160: f64, t749: f64, t3166: f64, t462: f64, t940: f64, t348: f64, t39: f64, t945: f64, t1784: f64, t343: f64, t1781: f64, t2953: f64, t2954: f64, t2961: f64, t34: f64, t4352: f64, t4355: f64, t47: f64, t739: f64, t8315: f64, t939: f64, t9481: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t11402 = 24.0_f64 * t11401;
    let t11403 = t3160 * t749;
    let t11404 = 240.0_f64 * t11403;
    let t11405 = t3166 * t749;
    let t11406 = 120.0_f64 * t11405;
    let t11411 = t462 * t940;
    let t11419 = t39 * t348;
    let t11422 = t462 * t945;
    let t11430 = 32.0_f64 * t1784 * t343;
    let t11432 = piecewise3(t44, 0.0_f64, 40.0_f64 / 81.0_f64 * t8315 * t739 * t2954 - 16.0_f64 / 9.0_f64 * t2953 * t34 * t11411 - 8.0_f64 / 9.0_f64 * t4352 * t9481 + 8.0_f64 / 3.0_f64 * t939 * t462 * t348 - 8.0_f64 * t4355 * t11419 + 8.0_f64 / 3.0_f64 * t4355 * t11422 + 4.0_f64 / 9.0_f64 * t1781 * t2961 - 16.0_f64 * t47 * t39 + t11430);
    (t11402, t11404, t11406, t11411, t11419, t11422, t11432)
}
