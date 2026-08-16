//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1203/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1203(t14103: f64, t14105: f64, t14108: f64, t14112: f64, t14157: f64, t14162: f64, t14164: f64, t14166: f64, t14170: f64, t14174: f64, t14176: f64, t14178: f64, t14183: f64) -> f64 {
    let t14184 = 0.03354522822333102_f64 * t14103 - 0.011181742741110338_f64 * t14105 + t14108 + t14112 + t14157 - t14162 + t14164 + t14166 + t14170 - t14174 + t14176 + t14178 + t14183;
    t14184
}
