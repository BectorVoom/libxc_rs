//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1085/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1085(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6519: f64, t6527: f64, t7116: f64, t7123: f64, t7124: f64, t7129: f64) -> f64 {
    let t11897 = 3.0646056102413666_f64 * t6527 - 3.0646056102413666_f64 * t6519 - 1.5323028051206833_f64 * t11556 + 1.5323028051206833_f64 * t11559 - 0.1018833958900598_f64 * t10962 + 1.0215352034137888_f64 * t11563 - 1.0215352034137888_f64 * t11566 - 0.3056501876701794_f64 * t11070 - 0.3056501876701794_f64 * t10954 - 0.3056501876701794_f64 * t10966 - 0.3056501876701794_f64 * t11062 + 1.5323028051206833_f64 * t11574 + 0.3056501876701794_f64 * t6327 + t7116 + t7123 - t7124 - t7129;
    t11897
}
