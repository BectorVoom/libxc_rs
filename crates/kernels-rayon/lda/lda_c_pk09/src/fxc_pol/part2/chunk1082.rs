//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1082/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1082(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6519: f64, t6527: f64, t7192: f64, t7199: f64, t7200: f64, t7205: f64) -> f64 {
    let t11834 = 1.532302805120685_f64 * t6527 - 1.532302805120685_f64 * t6519 - 0.7661514025603425_f64 * t11556 + 0.7661514025603425_f64 * t11559 - 0.05094169794502982_f64 * t10962 + 0.510767601706895_f64 * t11563 - 0.510767601706895_f64 * t11566 - 0.15282509383508946_f64 * t11070 - 0.15282509383508946_f64 * t10954 - 0.15282509383508946_f64 * t10966 - 0.15282509383508946_f64 * t11062 + 0.7661514025603425_f64 * t11574 + 0.15282509383508946_f64 * t6327 + t7192 + t7199 - t7200 - t7205;
    t11834
}
