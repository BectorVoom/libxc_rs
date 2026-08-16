//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 770/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk770(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3384: f64, t3388: f64, t3393: f64, t3398: f64, t3410: f64, t3411: f64, t3412: f64, t7851: f64, t7855: f64) -> f64 {
    let t7864 = 18.75_f64 * t7851 + 18.75_f64 * t7855 - 1.2466946262544771_f64 * t3335 - 0.8311297508363181_f64 * t3342 + 37.5_f64 * t3384 + 37.5_f64 * t3388 - 37.5_f64 * t3393 + t3398 + t3410 + t3411 - t3412 + 1.2466946262544771_f64 * t3317 + 1.2466946262544771_f64 * t3319;
    t7864
}
