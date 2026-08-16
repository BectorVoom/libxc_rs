//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1104/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1104(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6519: f64, t6527: f64, t6538: f64, t6545: f64, t6548: f64, t6563: f64) -> f64 {
    let t12217 = 6.416968383055361_f64 * t6527 - 6.416968383055361_f64 * t6519 - 3.2084841915276807_f64 * t11556 + 3.2084841915276807_f64 * t11559 - 0.21333333333333335_f64 * t10962 + 2.1389894610184537_f64 * t11563 - 2.1389894610184537_f64 * t11566 - 0.64_f64 * t11070 - 0.64_f64 * t10954 - 0.64_f64 * t10966 - 0.64_f64 * t11062 + 3.2084841915276807_f64 * t11574 + 0.64_f64 * t6327 + t6538 + t6545 - t6548 - t6563;
    t12217
}
