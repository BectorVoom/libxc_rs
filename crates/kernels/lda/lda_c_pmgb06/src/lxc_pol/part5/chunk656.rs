//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 656/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk656<F: Float>(t1380: F, t6390: F, t493: F, t1831: F, t851: F, t1981: F, t2545: F, t529: F, t2541: F, t337: F, t1915: F, t1: F, t1825: F, t1420: F, t2501: F, t2578: F, t477: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6391 = t1380 * t6390;
    let t6393 = 2.0 / 45.0 * t493 * t6391;
    let t6394 = t1831 * t851;
    let t6395 = t1380 * t6394;
    let t6397 = 4.0 / 45.0 * t1981 * t6395;
    let t6398 = t2545 * t529;
    let t6399 = t1380 * t6398;
    let t6401 = 2.0 / 45.0 * t493 * t6399;
    let t6402 = t2541 * t337;
    let t6403 = t1915 * t6402;
    let t6405 = 2.0 / 15.0 * t493 * t6403;
    let t6406 = t1825 * t1;
    let t6407 = t1915 * t6406;
    let t6409 = 8.0 / 45.0 * t1981 * t6407;
    let t6411 = 2.0 / 45.0 * t1420 * t2501;
    let t6412 = t2578 * t477;
    (t6391, t6393, t6394, t6395, t6397, t6398, t6399, t6401, t6402, t6403, t6405, t6406, t6407, t6409, t6411, t6412)
}
