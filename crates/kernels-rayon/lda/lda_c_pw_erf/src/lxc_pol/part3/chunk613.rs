//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 613/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk613(t1247: f64, t325: f64, t1458: f64, t56: f64, t3519: f64, t11: f64, t1124: f64, t174: f64, t177: f64, t25: f64, t3508: f64, t3510: f64, t3512: f64, t3520: f64, t3524: f64, t3528: f64, t3530: f64, t3532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3534 = t325 * t1247;
    let t3536 = t56 * t1458;
    let t3537 = t3536 * t3519;
    let t3538 = t11 * t3537;
    let t3540 = t1124 * t56;
    let t3542 = t174 * t3540 * t177;
    let t3543 = 0.11197407407407407_f64 * t3542;
    let t3544 = -0.022222222222222223_f64 * t3508 + 0.013333333333333334_f64 * t3510 + 0.0044444444444444444_f64 * t3512 - 0.002962962962962963_f64 * t25 * t3520 - 0.006666666666666667_f64 * t25 * t3524 - 0.035991666666666665_f64 * t3528 - 0.047988888888888886_f64 * t3530 + 0.035991666666666665_f64 * t3532 + 0.023994444444444443_f64 * t3534 - 0.03999074074074074_f64 * t3538 - t3543;
    (t3534, t3536, t3537, t3538, t3540, t3542, t3543, t3544)
}
