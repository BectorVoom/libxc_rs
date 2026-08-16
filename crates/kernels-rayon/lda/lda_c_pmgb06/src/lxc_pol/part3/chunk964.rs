//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 964/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk964(t350: f64, t365: f64, t5763: f64, t1271: f64, t2233: f64, t955: f64, t348: f64, t5760: f64, t1238: f64, t2210: f64, t110: f64, t5809: f64) -> (f64, f64, f64, f64, f64) {
    let t11370 = t365 * t5763 * t350;
    let t11373 = t1271 * t2233 * t955;
    let t11374 = 1.46904_f64 * t11373;
    let t11376 = t348 * t5760 * t350;
    let t11377 = 1.4615125_f64 * t11376;
    let t11379 = t1238 * t2210 * t955;
    let t11380 = 0.9743416666666667_f64 * t11379;
    let t11381 = t110 * t5809;
    (t11370, t11374, t11377, t11380, t11381)
}
