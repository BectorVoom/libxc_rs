//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1101/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1101(t350: f64, t4881: f64, t4886: f64, t1827: f64, t947: f64, t1822: f64, t4870: f64, t4641: f64, t4873: f64, t4858: f64, t138: f64, t4922: f64, t9175: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13345 = t350 * t4881;
    let t13347 = t350 * t4886;
    let t13370 = t947 * t1827;
    let t13372 = t947 * t1822;
    let t13374 = t350 * t4870;
    let t13376 = t4641 * t4873;
    let t13379 = t350 * t4858;
    let t13382 = t138 * t9175 * t4922;
    (t13345, t13347, t13370, t13372, t13374, t13376, t13379, t13382)
}
