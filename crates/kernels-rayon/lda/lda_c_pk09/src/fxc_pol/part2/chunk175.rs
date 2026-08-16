//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 175/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk175(t10: f64, t599: f64, t30: f64, t584: f64, t217: f64, t8: f64, t21: f64, t583: f64, t595: f64, t596: f64) -> (f64, f64, f64, f64) {
    let t600 = t599 * t10;
    let t601 = t584 * t30;
    let t604 = 1.0_f64 / t217;
    let t606 = t8 * t604 * t10;
    let t609 = -0.2071019728624174_f64 * t583 * t584 * t21 + 0.1855079159154325_f64 * t595 * t596 + 0.30174912456185365_f64 * t600 * t601 - 0.29107887321813086_f64 * t606 * t601;
    (t600, t604, t606, t609)
}
