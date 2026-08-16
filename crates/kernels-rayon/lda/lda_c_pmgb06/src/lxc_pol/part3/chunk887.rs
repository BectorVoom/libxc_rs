//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 887/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk887(t1498: f64, t607: f64, t500: f64, t1451: f64, t3226: f64, t1447: f64, t2984: f64, t2974: f64, t3300: f64, t2993: f64, t2873: f64, t1382: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9271 = t1498 * t607;
    let t9272 = t9271 * t500;
    let t9274 = t3226 * t1451;
    let t9291 = t1447 * t2984;
    let t9293 = t1447 * t2974;
    let t9295 = t1447 * t3300;
    let t9297 = t1447 * t2993;
    let t9311 = t1447 * t2873;
    let t9313 = t3226 * t1382;
    (t9271, t9272, t9274, t9291, t9293, t9295, t9297, t9311, t9313)
}
