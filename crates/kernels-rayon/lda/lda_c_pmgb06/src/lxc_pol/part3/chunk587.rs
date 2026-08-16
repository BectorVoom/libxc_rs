//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 587/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk587(t3010: f64, t3189: f64, t1436: f64, t439: f64, t1489: f64, t495: f64) -> (f64, f64, f64, f64) {
    let t3190 = t3189 * t3010;
    let t3191 = t1436 * t3190;
    let t3193 = 2.0_f64 / 9.0_f64 * t439 * t3191;
    let t3194 = t495 * t1489;
    (t3190, t3191, t3193, t3194)
}
