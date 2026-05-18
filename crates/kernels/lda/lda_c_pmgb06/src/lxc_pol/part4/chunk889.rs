//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 889/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk889<F: Float>(t2496: F, t2979: F, t493: F, t2088: F, t838: F, t1380: F, t1831: F, t851: F, t1981: F, t2545: F, t529: F, t2541: F, t337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6387 = t2979 * t2496;
    let t6389 = F::new(2.0) / F::new(45.0) * t493 * t6387;
    let t6390 = t838 * t2088;
    let t6391 = t1380 * t6390;
    let t6393 = F::new(2.0) / F::new(45.0) * t493 * t6391;
    let t6394 = t1831 * t851;
    let t6395 = t1380 * t6394;
    let t6397 = F::new(4.0) / F::new(45.0) * t1981 * t6395;
    let t6398 = t2545 * t529;
    let t6399 = t1380 * t6398;
    let t6401 = F::new(2.0) / F::new(45.0) * t493 * t6399;
    let t6402 = t2541 * t337;
    (t6387, t6389, t6390, t6391, t6393, t6394, t6395, t6397, t6398, t6399, t6401, t6402)
}
