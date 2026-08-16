//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1041/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1041(t3010: f64, t4649: f64, t36: f64, t453: f64, t350: f64, t4651: f64, t139: f64, t30: f64, t35: f64) -> (f64, f64, f64, f64) {
    let t12389 = t4649 * t3010;
    let t12391 = t36 * t453 * t12389;
    let t12393 = t350 * t4651;
    let t12396 = t30 * t35 * t139;
    (t12389, t12391, t12393, t12396)
}
