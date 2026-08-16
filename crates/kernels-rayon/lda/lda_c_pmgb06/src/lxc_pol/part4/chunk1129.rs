//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1129/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1129(t1309: f64, t769: f64, t395: f64, t4575: f64, t123: f64, t2822: f64, t868: f64, t14277: f64, t199: f64, t4435: f64, t722: f64, t14281: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14656 = t1309 * t769;
    let t14663 = t395 * t4575;
    let t14666 = t123 * t2822 * t868;
    let t14669 = t123 * t14277 * t199;
    let t14672 = t123 * t722 * t4435;
    let t14696 = t123 * t14281 * t199;
    (t14656, t14663, t14666, t14669, t14672, t14696)
}
