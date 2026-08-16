//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1097/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1097(t2918: f64, t518: f64, t12531: f64, t5138: f64, t2952: f64, t5077: f64, t5078: f64, t9885: f64, t9887: f64, t1179: f64, t132: f64, t441: f64, t4829: f64) -> (f64, f64, f64, f64, f64) {
    let t13068 = t518 * t2918;
    let t13071 = 2.0_f64 / 3.0_f64 * t5138 * t13068 * t12531;
    let t13074 = 2.0_f64 / 15.0_f64 * t5077 * t5078 * t2952;
    let t13075 = t9885 / 15.0_f64;
    let t13076 = t9887 / 15.0_f64;
    let t13079 = t132 * t1179 * t441 * t4829;
    (t13071, t13074, t13075, t13076, t13079)
}
