//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 888/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk888(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3384: f64, t3388: f64, t3393: f64, t4231: f64, t4234: f64, t4235: f64, t4236: f64, t7851: f64, t7855: f64) -> f64 {
    let t9363 = 4.59690841536205_f64 * t7851 + 4.59690841536205_f64 * t7855 - 0.3056501876701794_f64 * t3335 - 0.2037667917801196_f64 * t3342 + 9.1938168307241_f64 * t3384 + 9.1938168307241_f64 * t3388 - 9.1938168307241_f64 * t3393 + t4231 + t4234 + t4235 - t4236 + 0.3056501876701794_f64 * t3317 + 0.3056501876701794_f64 * t3319;
    t9363
}
