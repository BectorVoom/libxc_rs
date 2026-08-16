//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1018/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1018(t786: f64, t9599: f64, t1302: f64, t6580: f64, t5215: f64, t11906: f64, t11912: f64, t11917: f64, t11918: f64, t11919: f64, t11920: f64, t11921: f64, t11925: f64, t9250: f64, t9253: f64) -> (f64, f64, f64, f64) {
    let t11927 = 4.0_f64 / 15.0_f64 * t9599 * t786;
    let t11929 = 4.0_f64 / 5.0_f64 * t6580 * t1302;
    let t11931 = 4.0_f64 / 5.0_f64 * t5215 * t1302;
    let t11933 = t11906 - t11912 + t11917 - t11918 - t11919 + t11920 - t9250 - t11921 + t11925 + t11927 + t11929 + t11931 + 4.0_f64 * t9253;
    (t11927, t11929, t11931, t11933)
}
