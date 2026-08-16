//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 584/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk584(t2973: f64, t4364: f64, t1098: f64, t132: f64, t409: f64, t1063: f64, t3290: f64, t1076: f64, t3230: f64, t3233: f64, t1067: f64, t1095: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4365 = t4364 * t2973;
    let t4366 = t1098 * t4365;
    let t4368 = t409 * t132;
    let t4379 = t1063 * t3290 / 6.0_f64;
    let t4380 = t1076 * t3230;
    let t4382 = t1076 * t3233;
    let t4384 = t1095 * t1067;
    (t4365, t4366, t4368, t4379, t4380, t4382, t4384)
}
