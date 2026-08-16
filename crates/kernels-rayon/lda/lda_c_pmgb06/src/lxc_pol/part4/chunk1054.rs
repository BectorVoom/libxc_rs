//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1054/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1054(t342: f64, t569: f64, t99: f64, t1271: f64, t2229: f64, t2221: f64, t348: f64, t5772: f64, t1238: f64, t776: f64, t110: f64, t360: f64, t5775: f64) -> (f64, f64, f64, f64) {
    let t11303 = t99 * t569 * t342;
    let t11304 = t1271 * t2229 * t11303;
    let t11307 = t348 * t2221 * t5772;
    let t11310 = t1238 * t776 * t11303;
    let t11313 = t360 * t110 * t5775;
    (t11304, t11307, t11310, t11313)
}
