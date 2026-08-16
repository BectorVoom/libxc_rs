//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 889/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk889(t3323: f64, t3326: f64, t3424: f64, t3426: f64, t3428: f64, t4245: f64, t4252: f64, t4254: f64, t7870: f64, t7875: f64, t7879: f64, t7884: f64, t7888: f64) -> f64 {
    let t9375 = 0.2037667917801196_f64 * t3323 + 0.2037667917801196_f64 * t3326 + t4245 + 9.1938168307241_f64 * t7870 - 9.1938168307241_f64 * t7875 + 9.1938168307241_f64 * t7879 - 9.1938168307241_f64 * t7884 + 9.1938168307241_f64 * t7888 + 6.129211220482733_f64 * t3424 + 6.129211220482733_f64 * t3426 - 6.129211220482733_f64 * t3428 + t4252 + t4254;
    t9375
}
