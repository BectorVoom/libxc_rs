//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 685/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk685(t145: f64, t2906: f64, t2932: f64, t2934: f64, t2935: f64, t5745: f64, t5750: f64, t5768: f64, t5770: f64, t6046: f64, t6052: f64, t6080: f64) -> f64 {
    let t6083 = -0.031835665774679375_f64 * t6046 - t5768 - 0.06367133154935875_f64 * t5770 - 0.031835665774679375_f64 * t2906 - t2932 - t2934 + 0.31995040645307626_f64 * t2935 + 0.6399008129061525_f64 * t5745 - t5750 - 0.10665013548435875_f64 * t6052 + 0.05332506774217938_f64 * t145 * t6080;
    t6083
}
