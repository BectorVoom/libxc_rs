//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 240/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk240(t1005: f64, t101: f64, t89: f64, t650: f64, t96: f64, t93: f64) -> (f64, f64, f64, f64) {
    let t1006 = t101 * t1005;
    let t1007 = t1006 * t89;
    let t1010 = t96 * t650;
    let t1011 = t93 * t1010;
    (t1006, t1007, t1010, t1011)
}
