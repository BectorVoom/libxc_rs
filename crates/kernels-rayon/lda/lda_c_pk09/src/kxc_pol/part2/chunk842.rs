//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 842/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk842(t7731: f64, t839: f64, t164: f64, t7598: f64, t7602: f64, t7590: f64, t7578: f64, t2353: f64, t3836: f64, t119: f64, t120: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8600 = t839 * t7731;
    let t8602 = t164 * t7598;
    let t8604 = t164 * t7602;
    let t8606 = t164 * t7590;
    let t8608 = t164 * t7578;
    let t8612 = t2353 * t3836;
    let t8613 = t8612 * t119;
    let t8614 = t120 * t95;
    (t8600, t8602, t8604, t8606, t8608, t8612, t8613, t8614)
}
