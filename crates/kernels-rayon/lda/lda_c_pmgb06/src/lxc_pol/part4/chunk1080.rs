//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1080/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1080(t350: f64, t4673: f64, t4641: f64, t4669: f64, t4660: f64, t4646: f64, t4664: f64, t1865: f64, t947: f64, t1860: f64, t4651: f64, t1435: f64, t3092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12356 = t350 * t4673;
    let t12358 = t4641 * t4669;
    let t12360 = t350 * t4660;
    let t12362 = t350 * t4646;
    let t12364 = t350 * t4664;
    let t12366 = t947 * t1865;
    let t12368 = t947 * t1860;
    let t12393 = t350 * t4651;
    let t12397 = t1435 * t3092;
    (t12356, t12358, t12360, t12362, t12364, t12366, t12368, t12393, t12397)
}
