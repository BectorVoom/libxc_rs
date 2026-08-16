//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1125/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1125(t1347: f64, t1799: f64, t117: f64, t123: f64, t315: f64, t5689: f64, t1795: f64, t118: f64, t5575: f64, t2174: f64, t415: f64, t14239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14535 = t1799 * t1347;
    let t14539 = t123 * t315 * t5689 * t117;
    let t14541 = t1795 * t1347;
    let t14543 = t5575 * t118;
    let t14545 = t2174 * t415;
    let t14547 = t14239 * t118;
    (t14535, t14539, t14541, t14543, t14545, t14547)
}
