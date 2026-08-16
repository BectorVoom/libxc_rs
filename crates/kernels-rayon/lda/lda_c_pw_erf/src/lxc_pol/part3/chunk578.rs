//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 578/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk578(t1070: f64, t358: f64, t1064: f64, t1039: f64, t339: f64, t344: f64, t1037: f64, t2979: f64, t87: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3177 = t1070 * t358;
    let t3178 = 96.0_f64 * t3177;
    let t3179 = t1064 * t358;
    let t3180 = 60.0_f64 * t3179;
    let t3181 = t339 * t1039;
    let t3182 = 24.0_f64 * t3181;
    let t3183 = t344 * t1039;
    let t3184 = 24.0_f64 * t3183;
    let t3185 = t339 * t1037;
    let t3186 = 12.0_f64 * t3185;
    let t3187 = t344 * t1037;
    let t3188 = 12.0_f64 * t3187;
    let t3189 = t2979 * t87;
    let t3190 = t40 * t3189;
    (t3178, t3179, t3180, t3182, t3183, t3184, t3185, t3186, t3187, t3188, t3189, t3190)
}
