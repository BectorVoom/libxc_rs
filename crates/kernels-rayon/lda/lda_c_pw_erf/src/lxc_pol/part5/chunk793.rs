//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 793/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk793(t2660: f64, t611: f64, t225: f64, t6039: f64, t231: f64, t5365: f64, t5373: f64, t5380: f64, t5399: f64, t5411: f64, t5423: f64, t6995: f64, t7001: f64, t7006: f64, t7009: f64, t7011: f64, t7014: f64, t7015: f64, t7018: f64, t7020: f64) -> (f64, f64, f64) {
    let t7278 = t2660 * t611;
    let t7280 = t6039 * t225;
    let t7283 = -t6995 - t7001 + t7006 + t7009 - t7011 - t7014 - t7015 - t5365 + t5373 - t5380 + t5399 + 4.0_f64 / 3.0_f64 * t7278 + 4.0_f64 / 3.0_f64 * t7280 * t231 + t5411 - t5423 - t7018 + t7020;
    (t7278, t7280, t7283)
}
