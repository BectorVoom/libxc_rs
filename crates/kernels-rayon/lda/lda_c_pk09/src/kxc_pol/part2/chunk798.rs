//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 798/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk798(t3319: f64, t3323: f64, t3326: f64, t3870: f64, t7896: f64, t7919: f64, t7923: f64, t7926: f64, t7928: f64, t7931: f64, t7935: f64, t7939: f64, t7942: f64) -> f64 {
    let t8026 = 1.5625_f64 * t3319 + 1.0416666666666667_f64 * t3323 + 1.0416666666666667_f64 * t3326 + t3870 + 3.125_f64 * t7896 + 1.5625_f64 * t7919 + 1.5625_f64 * t7923 + 1.5625_f64 * t7926 + 1.5625_f64 * t7928 + 1.5625_f64 * t7931 + 1.5625_f64 * t7935 + 1.0416666666666667_f64 * t7939 + 1.0416666666666667_f64 * t7942;
    t8026
}
