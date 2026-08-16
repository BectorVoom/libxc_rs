//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 579/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk579(t390: f64, t960: f64, t40: f64, t3168: f64, t3170: f64, t3172: f64, t3174: f64, t3176: f64, t3178: f64, t3180: f64, t3182: f64, t3184: f64, t3186: f64, t3188: f64, t3190: f64) -> (f64, f64, f64) {
    let t3191 = t960 * t390;
    let t3192 = t40 * t3191;
    let t3193 = 3.0_f64 * t3192;
    let t3194 = -t3168 + t3170 + t3172 - t3174 + t3176 - t3178 + t3180 + t3182 - t3184 + t3186 - t3188 + t3190 + t3193;
    (t3191, t3192, t3194)
}
