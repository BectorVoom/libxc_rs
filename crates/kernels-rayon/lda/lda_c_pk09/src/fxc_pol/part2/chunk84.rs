//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 84/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk84(t242: f64, t8: f64, t17: f64, t231: f64, t237: f64, t240: f64, t42: f64) -> (f64, f64, f64, f64, f64) {
    let t243 = t8 * t242;
    let t246 = -113.69336978972719_f64 + 6.001255378196778_f64 * t17 + 12.335328239599177_f64 * t231 - 5.687617677680484_f64 * t42 + 0.17701513906783214_f64 * t237 + 5.4944839533438375e-05_f64 * t240 * t243;
    let t251 = -109.7426349321691_f64 + 4.066578236106061_f64 * t17 + 13.600858284347709_f64 * t231 - 6.005077522251017_f64 * t42 + 0.1875_f64 * t237;
    let t252 = 1.0_f64 / t251;
    let t253 = t246 * t252;
    (t243, t246, t251, t252, t253)
}
