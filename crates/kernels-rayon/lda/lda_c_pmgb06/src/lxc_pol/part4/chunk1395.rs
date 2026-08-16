//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1395/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1395(t11944: f64, t16077: f64, t16083: f64, t16087: f64, t16090: f64, t16092: f64, t16094: f64, t16095: f64, t16099: f64, t9408: f64, t9410: f64, t9412: f64, t9417: f64, t9418: f64, t9422: f64) -> f64 {
    let t18203 = -0.13298177777777778_f64 * t11944 - t16077 - t16083 - t16087 + t16090 - t16092 - t16094 - t16095 - t16099 - t9408 + t9410 + t9412 - t9417 + 4.0_f64 / 9.0_f64 * t9418 + 4.0_f64 / 3.0_f64 * t9422;
    t18203
}
