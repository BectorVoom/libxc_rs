//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 822/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk822(t3158: f64, t3161: f64, t3169: f64, t3171: f64, t3173: f64, t3179: f64, t3183: f64, t3185: f64, t3187: f64, t3133: f64, t3139: f64, t3151: f64, t3157: f64, t3168: f64, t3176: f64, t3192: f64) -> f64 {
    let t5707 = 48.0_f64 * t3158;
    let t5708 = 80.0_f64 * t3161;
    let t5709 = 12.0_f64 * t3169;
    let t5710 = 24.0_f64 * t3171;
    let t5711 = 32.0_f64 * t3173;
    let t5712 = 40.0_f64 * t3179;
    let t5713 = 16.0_f64 * t3183;
    let t5714 = 4.0_f64 * t3185;
    let t5715 = 4.0_f64 * t3187;
    let t5716 = t3133 - t3139 + t3151 - t3157 + t5707 + t5708 - t3168 - t5709 - t5710 - t5711 + t3176 + t5712 - t5713 - t5714 - t5715 + t3192;
    t5716
}
