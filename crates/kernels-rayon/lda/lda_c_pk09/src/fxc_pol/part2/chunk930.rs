//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 930/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk930(t1290: f64, t9836: f64, t1402: f64, t9602: f64, t1287: f64, t741: f64, t7766: f64, t93: f64, t10: f64, t1214: f64, t407: f64, t130: f64, t9739: f64) -> (f64, f64, f64, f64, f64) {
    let t9837 = t1290 * t9836;
    let t9839 = t1402 * t9602;
    let t9840 = t9839 * t1287;
    let t9842 = t741 * t7766;
    let t9843 = t93 * t9842;
    let t9846 = t1214 * t10;
    let t9847 = t407 * t9846;
    let t9850 = t130 * t9739;
    (t9837, t9840, t9843, t9847, t9850)
}
