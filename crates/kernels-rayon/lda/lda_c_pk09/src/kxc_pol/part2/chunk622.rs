//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 622/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk622(t390: f64, t391: f64, t387: f64, t5039: f64, t68: f64, t70: f64) -> (f64, f64, f64, f64) {
    let t5141 = t390 * t390;
    let t5143 = 1.0_f64 / t391 / t5141;
    let t5144 = t387 * t5143;
    let t5150 = 0.505765839233979_f64 * t5039;
    let t5153 = t68 * t70;
    (t5141, t5144, t5150, t5153)
}
