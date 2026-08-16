//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 931/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk931(t93: f64, t9850: f64, t1345: f64, t9836: f64, t1434: f64, t2649: f64, t1348: f64, t1388: f64, t2674: f64, t747: f64, t2520: f64, t1481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9851 = t93 * t9850;
    let t9854 = t1345 * t9836;
    let t9856 = t1434 * t2649;
    let t9857 = t1348 * t9856;
    let t9860 = t1388 * t747 * t2674;
    let t9862 = t747 * t2520;
    let t9863 = t1481 * t9862;
    (t9851, t9854, t9857, t9860, t9862, t9863)
}
