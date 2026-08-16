//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1064/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1064(t2257: f64, t26: f64, t329: f64, t1322: f64, t5882: f64, t301: f64, t413: f64, t5567: f64, t642: f64, t794: f64, t113: f64, t1329: f64, t1808: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11645 = t5882 * t1322;
    let t11674 = t5567 * t413 * t301;
    let t11676 = t642 * t794;
    let t11678 = t11676 * t113 * t301;
    let t11698 = t1329 * t1808;
    (t11639, t11640, t11645, t11674, t11676, t11678, t11698)
}
