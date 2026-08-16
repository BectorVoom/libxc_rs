//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 997/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk997(t145: f64, t6039: f64, t164: f64, t479: f64, t7045: f64, t2660: f64, t610: f64, t10605: f64, t2543: f64, t571: f64, t2171: f64, t5397: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15483 = t145 * t6039;
    let t15484 = t15483 * t164;
    let t15486 = t7045 * t479;
    let t15501 = t2660 * t610;
    let t15521 = t571 * t10605 * t2543;
    let t15525 = t2171 * t5397;
    (t15483, t15484, t15486, t15501, t15521, t15525)
}
