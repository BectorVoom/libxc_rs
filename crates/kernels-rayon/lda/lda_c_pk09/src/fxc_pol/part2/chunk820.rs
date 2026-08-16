//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 820/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk820(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3384: f64, t3388: f64, t3393: f64, t3789: f64, t3792: f64, t3793: f64, t3794: f64, t7851: f64, t7855: f64) -> f64 {
    let t8286 = 0.6806222787477182_f64 * t7851 + 0.6806222787477182_f64 * t7855 - 0.04525483399593904_f64 * t3335 - 0.03016988933062603_f64 * t3342 + 1.3612445574954364_f64 * t3384 + 1.3612445574954364_f64 * t3388 - 1.3612445574954364_f64 * t3393 + t3789 + t3792 + t3793 - t3794 + 0.04525483399593904_f64 * t3317 + 0.04525483399593904_f64 * t3319;
    t8286
}
