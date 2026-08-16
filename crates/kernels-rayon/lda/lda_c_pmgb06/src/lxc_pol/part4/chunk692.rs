//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 692/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk692(t211: f64, t410: f64, t209: f64, t1684: f64, t591: f64, t1688: f64, t125: f64, t208: f64, t586: f64) -> (f64, f64, f64, f64, f64) {
    let t4103 = t211 * t410;
    let t4105 = 8.0_f64 / 81.0_f64 * t209 * t4103;
    let t4106 = t1684 * t591;
    let t4108 = t1688 * t591;
    let t4111 = t586 * t125 * t208;
    (t4103, t4105, t4106, t4108, t4111)
}
