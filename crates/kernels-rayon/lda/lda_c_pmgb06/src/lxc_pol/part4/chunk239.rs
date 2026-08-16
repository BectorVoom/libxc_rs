//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 239/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk239(t302: f64, t707: f64, t113: f64, t301: f64, t398: f64, t413: f64, t83: f64, t297: f64, t395: f64) -> (f64, f64, f64, f64, f64) {
    let t709 = 0.019957056683757683_f64 * t707 * t302;
    let t711 = t398 * t113 * t301;
    let t715 = t83 * t413 * t301;
    let t717 = 0.01197423401025461_f64 * t297 * t715;
    let t718 = t395 * t83;
    (t709, t711, t715, t717, t718)
}
