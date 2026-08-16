//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 785/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk785(t2012: f64, t5211: f64, t3226: f64, t835: f64, t1447: f64, t1977: f64, t1423: f64, t1963: f64, t607: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5212 = t5211 * t2012;
    let t5213 = 2.0_f64 / 27.0_f64 * t5212;
    let t5215 = 4.0_f64 / 135.0_f64 * t3226 * t835;
    let t5217 = 4.0_f64 / 135.0_f64 * t1447 * t1977;
    let t5219 = 4.0_f64 / 135.0_f64 * t1423 * t1963;
    let t5220 = t801 * t607;
    (t5212, t5213, t5215, t5217, t5219, t5220)
}
