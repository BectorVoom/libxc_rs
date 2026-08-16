//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 776/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk776(t3965: f64, t5138: f64, t1251: f64, t4722: f64, t348: f64, t5136: f64, t1458: f64, t197: f64, t1245: f64, t3975: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5140 = 16.0_f64 / 45.0_f64 * t3965 * t5138;
    let t5141 = t4722 * t1251;
    let t5142 = t5136 * t348;
    let t5143 = t5141 * t5142;
    let t5145 = 32.0_f64 / 45.0_f64 * t3965 * t5143;
    let t5146 = t1458 * t197;
    let t5147 = t5146 * t1245;
    let t5148 = t5147 * t5142;
    let t5150 = 16.0_f64 / 27.0_f64 * t3965 * t5148;
    let t5151 = t3975 * t833;
    (t5140, t5141, t5143, t5145, t5146, t5147, t5148, t5150, t5151)
}
