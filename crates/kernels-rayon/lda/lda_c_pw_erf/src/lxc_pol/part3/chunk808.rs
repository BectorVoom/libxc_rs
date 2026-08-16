//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 808/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk808(t426: f64, t5521: f64, t3234: f64, t739: f64, t1558: f64, t34: f64, t3243: f64, t743: f64, t1563: f64, t1820: f64, t1823: f64, t1826: f64, t1829: f64, t39: f64, t406: f64, t408: f64, t4356: f64, t4371: f64, t462: f64, t940: f64, t945: f64, t951: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5523 = t426 * t5521 / 3.0_f64;
    let t5524 = t3234 * t739;
    let t5527 = t1558 * t34;
    let t5536 = t3243 * t743;
    let t5539 = t1563 * t34;
    let t5548 = 4.0_f64 / 27.0_f64 * t5524 * t940 - 4.0_f64 / 9.0_f64 * t5527 * t4356 - t1820 * t945 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t406 * t462 - 2.0_f64 * t1823 * t39 + 4.0_f64 / 27.0_f64 * t5536 * t951 + 4.0_f64 / 9.0_f64 * t5539 * t4371 - t1826 * t954 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t408 * t462 + 2.0_f64 * t1829 * t39;
    (t5523, t5524, t5527, t5536, t5539, t5548)
}
