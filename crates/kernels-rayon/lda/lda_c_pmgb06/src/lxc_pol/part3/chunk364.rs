//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 364/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk364(t399: f64, t415: f64, t398: f64, t409: f64, t419: f64, t421: f64, t117: f64, t1184: f64) -> (f64, f64, f64, f64) {
    let t1341 = t399 * t415;
    let t1343 = t409 * t398;
    let t1345 = t1343 * t419 * t421;
    let t1347 = t1184 * t117;
    (t1341, t1343, t1345, t1347)
}
