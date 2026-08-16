//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1108/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1108(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6637: f64, t6639: f64, t7446: f64, t7453: f64, t7454: f64, t7459: f64) -> f64 {
    let t12275 = t6639 - t6637 - 4.0_f64 * t11556 + 4.0_f64 * t11559 - 0.2738064645187903_f64 * t10962 + 2.6666666666666665_f64 * t11563 - 2.6666666666666665_f64 * t11566 - 0.821419393556371_f64 * t11070 - 0.821419393556371_f64 * t10954 - 0.821419393556371_f64 * t10966 - 0.821419393556371_f64 * t11062 + 4.0_f64 * t11574 + 0.821419393556371_f64 * t6327 + t7446 + t7453 - t7454 - t7459;
    t12275
}
