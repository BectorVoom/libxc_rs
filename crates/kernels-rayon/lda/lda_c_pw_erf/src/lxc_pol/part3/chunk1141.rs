//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1141/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1141(t2014: f64, t3742: f64, t3854: f64, t4684: f64, t571: f64, t2967: f64, t4670: f64, t1319: f64, t2023: f64, t4624: f64, t519: f64, t5237: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13364 = 16.0_f64 / 15.0_f64 * t3742 * t2014;
    let t13366 = t571 * t3854 * t4684;
    let t13367 = 16.0_f64 / 15.0_f64 * t13366;
    let t13368 = t4670 * t2967;
    let t13371 = 32.0_f64 / 15.0_f64 * t571 * t1319 * t13368;
    let t13373 = 8.0_f64 / 15.0_f64 * t3742 * t2023;
    let t13375 = t519 * t5237 * t4624;
    (t13364, t13367, t13368, t13371, t13373, t13375)
}
