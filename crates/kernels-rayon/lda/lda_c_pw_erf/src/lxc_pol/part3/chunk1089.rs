//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1089/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1089(t9721: f64, t9725: f64, t9737: f64, t9905: f64, t493: f64, t9946: f64, t9909: f64, t1508: f64, t2134: f64, t9923: f64, t9925: f64, t9928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12740 = 16.0_f64 / 27.0_f64 * t9721;
    let t12741 = 8.0_f64 / 27.0_f64 * t9725;
    let t12742 = 32.0_f64 / 45.0_f64 * t9737;
    let t12743 = 8.0_f64 / 15.0_f64 * t9905;
    let t12745 = 4.0_f64 / 5.0_f64 * t493 * t9946;
    let t12746 = 16.0_f64 / 135.0_f64 * t9909;
    let t12747 = t1508 * t2134;
    let t12748 = 4.0_f64 / 15.0_f64 * t12747;
    let t12749 = 4.0_f64 / 45.0_f64 * t9923;
    let t12750 = 16.0_f64 / 45.0_f64 * t9925;
    let t12751 = 4.0_f64 / 15.0_f64 * t9928;
    (t12740, t12741, t12742, t12743, t12745, t12746, t12748, t12749, t12750, t12751)
}
