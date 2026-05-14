//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 985/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk985<F: Float>(t350: F, t4858: F, t138: F, t4922: F, t9175: F, t1461: F, t2911: F, t12396: F, t12547: F, t2918: F, t495: F, t1464: F, t165: F, t1832: F, t8337: F, t1476: F, t1830: F, t2923: F) -> (F, F, F, F, F, F, F) {
    let t13379 = t350 * t4858;
    let t13382 = t138 * t9175 * t4922;
    let t13384 = t1461 * t2911;
    let t13386 = t12396 * t13384 * t12547;
    let t13388 = t495 * t2918;
    let t13390 = t12396 * t13388 * t12547;
    let t13392 = t165 * t1464;
    let t13394 = t12396 * t13392 * t12547;
    let t13399 = t8337 * t1832;
    let t13402 = t1830 * t1476 * t2923;
    (t13379, t13382, t13386, t13390, t13394, t13399, t13402)
}
