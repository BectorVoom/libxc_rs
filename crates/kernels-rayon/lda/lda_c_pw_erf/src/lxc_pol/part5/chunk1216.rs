//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1216/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1216(t21942: f64, t13479: f64, t21915: f64, t21917: f64, t21919: f64, t21921: f64, t21923: f64, t21926: f64, t21928: f64, t21932: f64, t21936: f64, t21938: f64, t21940: f64) -> (f64, f64) {
    let t21943 = 8.0_f64 / 15.0_f64 * t21942;
    let t21944 = -t21915 + t21917 - t21919 - t21921 + t21923 - t21926 + t21928 - t13479 - t21932 + t21936 - t21938 - t21940 - t21943;
    (t21943, t21944)
}
