//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 543/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk543(t2803: f64, t3: f64, t1338: f64, t415: f64, t1139: f64, t118: f64, t718: f64, t1166: f64, t81: f64) -> (f64, f64, f64, f64, f64) {
    let t2804 = t3 * t2803;
    let t2807 = t1338 * t415;
    let t2809 = t1139 * t118;
    let t2812 = 0.1890324433388467_f64 * t718 * t415;
    let t2813 = t81 * t1166;
    (t2804, t2807, t2809, t2812, t2813)
}
