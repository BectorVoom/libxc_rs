//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 594/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk594(t133: f64, t3161: f64, t131: f64, t4645: f64, t568: f64, t736: f64, t735: f64, t197: f64, t1121: f64, t1124: f64, t167: f64, t125: f64, t658: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4646 = t133 * t3161;
    let t4647 = t131 * t4646;
    let t4649 = 7.108175748183851_f64 * t4645 * t4647;
    let t4650 = t568 * t736;
    let t4652 = 6.31837844283009_f64 * t735 * t4650;
    let t4654 = t197 * t197;
    let t4655 = 1.0_f64 / t4654;
    let t4660 = t1124 * t1121;
    let t4667 = t167 * t167;
    let t4668 = 1.0_f64 / t4667;
    let t4673 = t658 * t125;
    (t4649, t4652, t4655, t4660, t4668, t4673)
}
