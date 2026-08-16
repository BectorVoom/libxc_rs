//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 772/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk772(t4628: f64, t538: f64, t2092: f64, t331: f64, t25: f64, t3472: f64, t3473: f64, t3493: f64, t3508: f64, t3510: f64, t3512: f64, t3543: f64, t4600: f64, t4604: f64, t4607: f64, t4617: f64, t4630: f64, t5072: f64, t5076: f64, t5084: f64) -> (f64, f64) {
    let t5087 = t538 * t4628;
    let t5093 = 0.017777777777777778_f64 * t331 * t2092;
    let t5094 = 0.057777777777777775_f64 * t5072 - 0.015996296296296297_f64 * t4600 + 0.2639388888888889_f64 * t4607 - 0.007407407407407408_f64 * t5076 - t3472 - t3543 - 0.008888888888888889_f64 * t3473 - 0.023994444444444443_f64 * t3493 - 0.014814814814814815_f64 * t3508 + 0.0044444444444444444_f64 * t3510 + 0.0014814814814814814_f64 * t3512 - 0.047988888888888886_f64 * t4604 + 0.013333333333333334_f64 * t25 * t5084 - 0.04_f64 * t25 * t5087 - 0.21595_f64 * t4630 + 0.14396666666666666_f64 * t4617 - t5093;
    (t5087, t5094)
}
