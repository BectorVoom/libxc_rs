//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 233/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk233(t123: f64, t199: f64, t722: f64, t125: f64, t398: f64, t290: f64, t395: f64, t100: f64, t394: f64) -> (f64, f64, f64, f64) {
    let t725 = 0.053059442957798957_f64 * t123 * t722 * t199;
    let t726 = t125 * t398;
    let t734 = 0.10665013548435875_f64 * t395 * t290;
    let t740 = 1.0_f64 / t100 / t394;
    (t725, t726, t734, t740)
}
