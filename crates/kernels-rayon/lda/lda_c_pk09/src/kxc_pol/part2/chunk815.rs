//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 815/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk815(t3323: f64, t3326: f64, t3424: f64, t3426: f64, t3428: f64, t3960: f64, t3967: f64, t3969: f64, t7870: f64, t7875: f64, t7879: f64, t7884: f64, t7888: f64) -> f64 {
    let t8214 = 0.337177226155986_f64 * t3323 + 0.337177226155986_f64 * t3326 + t3960 + 12.0_f64 * t7870 - 12.0_f64 * t7875 + 12.0_f64 * t7879 - 12.0_f64 * t7884 + 12.0_f64 * t7888 + 8.0_f64 * t3424 + 8.0_f64 * t3426 - 8.0_f64 * t3428 + t3967 + t3969;
    t8214
}
