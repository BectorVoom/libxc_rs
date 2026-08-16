//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 365/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk365(t1347: f64, t84: f64, t1186: f64, t418: f64, t421: f64, t299: f64, t934: f64) -> (f64, f64, f64) {
    let t1349 = 0.031505407223141116_f64 * t84 * t1347;
    let t1352 = 0.003950778065781896_f64 * t418 * t1186 * t421;
    let t1354 = t934 * t299;
    (t1349, t1352, t1354)
}
