//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 827/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk827(t3319: f64, t3323: f64, t3326: f64, t3598: f64, t7896: f64, t7919: f64, t7923: f64, t7926: f64, t7928: f64, t7931: f64, t7935: f64, t7939: f64, t7942: f64) -> f64 {
    let t8373 = 2.0_f64 * t3319 + 4.0_f64 / 3.0_f64 * t3323 + 4.0_f64 / 3.0_f64 * t3326 + t3598 + 4.0_f64 * t7896 + 2.0_f64 * t7919 + 2.0_f64 * t7923 + 2.0_f64 * t7926 + 2.0_f64 * t7928 + 2.0_f64 * t7931 + 2.0_f64 * t7935 + 4.0_f64 / 3.0_f64 * t7939 + 4.0_f64 / 3.0_f64 * t7942;
    t8373
}
