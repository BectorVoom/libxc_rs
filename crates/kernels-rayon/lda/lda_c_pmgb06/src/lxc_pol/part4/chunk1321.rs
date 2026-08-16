//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1321/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1321(t161: f64, t166: f64, t17321: f64, t17333: f64, t17347: f64, t17361: f64, t176: f64, t1499: f64, t2555: f64, t486: f64, t6833: f64, t5051: f64, t802: f64) -> (f64, f64, f64, f64) {
    let t17367 = t161 * t166 * (t17321 + t17333 + t17347 + t17361) * t176 / 30.0_f64;
    let t17369 = t1499 * t2555 / 30.0_f64;
    let t17371 = t486 * t6833 / 15.0_f64;
    let t17372 = t802 * t5051;
    (t17367, t17369, t17371, t17372)
}
