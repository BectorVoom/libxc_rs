//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1182/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1182(t103: f64, t11997: f64, t12563: f64, t12568: f64, t13407: f64, t14162: f64, t14170: f64, t14181: f64, t14183: f64, t14185: f64, t14187: f64, t14189: f64, t1576: f64, t2060: f64, t2923: f64, t2932: f64, t3358: f64, t525: f64, t9522: f64, t9530: f64, t9532: f64, t9534: f64, t9537: f64, t9552: f64, t9554: f64) -> f64 {
    let t14198 = 0.03732469135802469_f64 * t13407 + 0.28444444444444444_f64 * t14162 + 0.013333333333333334_f64 * t2060 * t1576 * t2923 - 0.08_f64 * t2060 * t525 * t2932 + 0.019753086419753086_f64 * t14170 + 0.035555555555555556_f64 * t103 * t3358 * t12563 - 0.08_f64 * t2060 * t1576 * t12568 + 0.24_f64 * t2060 * t525 * t11997 + 0.044444444444444446_f64 * t14181 - 0.007407407407407408_f64 * t14183 - 0.02666666666666667_f64 * t14185 + 0.3466666666666667_f64 * t14187 + 0.0044444444444444444_f64 * t14189 - 0.047988888888888886_f64 * t9522 - 0.047988888888888886_f64 * t9530 - 0.03199259259259259_f64 * t9532 + 0.011997222222222222_f64 * t9534 + 0.013330246913580247_f64 * t9537 + 0.11197407407407407_f64 * t9552 + 0.07198333333333333_f64 * t9554;
    t14198
}
