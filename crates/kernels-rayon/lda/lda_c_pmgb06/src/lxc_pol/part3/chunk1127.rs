//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1127/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1127(t350: f64, t4858: f64, t138: f64, t4922: f64, t9175: f64, t1461: f64, t2911: f64, t12396: f64, t12547: f64, t2918: f64, t495: f64, t1464: f64, t165: f64) -> (f64, f64, f64, f64, f64) {
    let t13379 = t350 * t4858;
    let t13382 = t138 * t9175 * t4922;
    let t13384 = t1461 * t2911;
    let t13386 = t12396 * t13384 * t12547;
    let t13388 = t495 * t2918;
    let t13390 = t12396 * t13388 * t12547;
    let t13392 = t165 * t1464;
    (t13379, t13382, t13386, t13390, t13392)
}
