//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1037/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1037(t1992: f64, t3709: f64, t1472: f64, t4869: f64, t4675: f64, t954: f64, t4868: f64, t571: f64, t219: f64, t4048: f64, t473: f64, t34: f64, t3589: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12150 = 4.0_f64 / 9.0_f64 * t3709 * t1992;
    let t12152 = 16.0_f64 / 9.0_f64 * t1472 * t4869;
    let t12153 = t4675 * t954;
    let t12156 = 8.0_f64 / 9.0_f64 * t571 * t4868 * t12153;
    let t12158 = t473 * t4048 * t219;
    let t12160 = t3589 * t34 * t951;
    (t12150, t12152, t12153, t12156, t12158, t12160)
}
