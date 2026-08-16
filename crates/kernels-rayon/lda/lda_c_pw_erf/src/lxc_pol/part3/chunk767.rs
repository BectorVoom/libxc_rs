//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 767/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk767(t2062: f64, t5021: f64, t830: f64, t933: f64, t2061: f64, t25: f64, t4657: f64, t4661: f64, t4663: f64, t4668: f64, t4678: f64, t4682: f64, t4686: f64, t4998: f64, t5000: f64, t5001: f64, t5004: f64, t5007: f64, t5010: f64, t5013: f64, t5017: f64) -> f64 {
    let t5022 = t5021 * t2062;
    let t5024 = t933 * t830;
    let t5028 = -0.21595_f64 * t4686 - t4998 + t5000 - 0.0022222222222222222_f64 * t25 * t5001 - 0.002962962962962963_f64 * t25 * t5004 + 0.008888888888888889_f64 * t2061 * t5007 + 0.013333333333333334_f64 * t25 * t5010 - 0.05333333333333334_f64 * t2061 * t5013 - 0.047988888888888886_f64 * t4661 + t5017 - 0.023994444444444443_f64 * t4682 - 0.03999074074074074_f64 * t4668 + 0.09597777777777777_f64 * t4678 - 0.057777777777777775_f64 * t5022 - 0.007407407407407408_f64 * t5024 - 0.015996296296296297_f64 * t4657 - 0.2639388888888889_f64 * t4663;
    t5028
}
