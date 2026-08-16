//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 698/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk698(t566: f64, t718: f64, t199: f64, t2813: f64, t1329: f64, t1200: f64, t391: f64, t26: f64, t386: f64, t329: f64, t1322: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4214 = t718 * t566;
    let t4216 = t2813 * t199;
    let t4218 = t1329 * t566;
    let t4220 = t391 * t1200;
    let t4230 = t26 * t386;
    let t4231 = t329 * t4230;
    let t4232 = t1322 * t73;
    (t4214, t4216, t4218, t4220, t4230, t4231, t4232)
}
