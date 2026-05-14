//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 918/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk918<F: Float>(t19340: F, t36: F, t506: F, t2389: F, t4851: F, t1830: F, t1414: F, t337: F, t7300: F, t1464: F, t1476: F, t1820: F, t5974: F, t4865: F, t350: F, t7602: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19342 = t36 * t506 * t19340;
    let t19344 = t4851 * t2389;
    let t19346 = t1830 * t506 * t19344;
    let t19349 = t1414 * t7300 * t337;
    let t19351 = t36 * t506 * t19349;
    let t19354 = t1464 * t7300 * t337;
    let t19356 = t36 * t1476 * t19354;
    let t19358 = t1820 * t5974;
    let t19360 = t36 * t1476 * t19358;
    let t19362 = t4865 * t2389;
    let t19364 = t1830 * t1476 * t19362;
    let t19366 = t350 * t7602;
    (t19342, t19344, t19346, t19349, t19351, t19354, t19356, t19358, t19360, t19362, t19364, t19366)
}
