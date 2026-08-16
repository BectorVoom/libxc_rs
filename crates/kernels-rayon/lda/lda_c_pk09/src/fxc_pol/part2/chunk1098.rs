//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1098/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1098(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6519: f64, t6527: f64, t6642: f64, t6649: f64, t6650: f64, t6655: f64) -> f64 {
    let t12113 = 4.0_f64 * t6527 - 4.0_f64 * t6519 - 2.0_f64 * t11556 + 2.0_f64 * t11559 - 0.168588613077993_f64 * t10962 + 1.3333333333333333_f64 * t11563 - 1.3333333333333333_f64 * t11566 - 0.505765839233979_f64 * t11070 - 0.505765839233979_f64 * t10954 - 0.505765839233979_f64 * t10966 - 0.505765839233979_f64 * t11062 + 2.0_f64 * t11574 + 0.505765839233979_f64 * t6327 + t6642 + t6649 - t6650 - t6655;
    t12113
}
