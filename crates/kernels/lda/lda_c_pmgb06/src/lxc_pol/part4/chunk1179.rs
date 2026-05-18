//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1179/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1179<F: Float>(t1420: F, t6495: F, t1444: F, t6761: F, t493: F, t5447: F, t6760: F, t1414: F, t337: F, t5974: F, t1915: F, t2948: F, t439: F, t6774: F) -> (F, F, F, F, F, F) {
    let t15496 = F::new(8.0) / F::new(45.0) * t1420 * t6495;
    let t15498 = F::new(4.0) / F::new(45.0) * t1444 * t6761;
    let t15501 = F::new(4.0) / F::new(45.0) * t493 * t5447 * t6760;
    let t15503 = t1414 * t5974 * t337;
    let t15506 = F::new(4.0) / F::new(45.0) * t493 * t1915 * t15503;
    let t15509 = F::new(2.0) / F::new(45.0) * t439 * t2948 * t6774;
    (t15496, t15498, t15501, t15503, t15506, t15509)
}
