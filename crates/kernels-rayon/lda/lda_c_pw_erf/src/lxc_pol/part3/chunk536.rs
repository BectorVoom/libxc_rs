//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 536/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk536(t2719: f64, t918: f64, t1124: f64, t119: f64, t321: f64, t11: f64, t2: f64, t39: f64, t928: f64, t328: f64, t1953: f64, t2061: f64, t2717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2720 = t918 * t2719;
    let t2722 = t119 * t1124;
    let t2723 = t321 * t2722;
    let t2726 = 1.0_f64/pow_3_2(t11);
    let t2727 = t2726 * t2;
    let t2728 = t2727 * t39;
    let t2730 = t928 * t2719;
    let t2732 = t328 * t2722;
    let t2735 = -3.4523333333333333_f64 * t2717 + 2.3015555555555554_f64 * t2720 - 2.6851481481481483_f64 * t2723 - 0.9393222222222222_f64 * t1953 + 0.073355_f64 * t2728 - 0.14671_f64 * t2730 - 0.17116166666666666_f64 * t2732 - 0.36793333333333333_f64 * t2061;
    (t2720, t2723, t2727, t2728, t2730, t2732, t2735)
}
