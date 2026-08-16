//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 591/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk591(t102: f64, t3222: f64, t436: f64, t120: f64, t3251: f64, t125: f64, t917: f64, t128: f64, t2: f64, t39: f64, t1697: f64, t411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3302 = 17.53815_f64 * t102 * t436 * t3222;
    let t3305 = 2.923025_f64 * t102 * t120 * t3251;
    let t3309 = t125 * t917;
    let t3310 = t128 * t2;
    let t3313 = 0.3264533333333333_f64 * t3309 * t3310 * t39;
    let t3314 = t1697 * t411;
    (t3302, t3305, t3309, t3310, t3313, t3314)
}
