//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1304/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1304<F: Float>(t350: F, t6821: F, t1464: F, t337: F, t5974: F, t1476: F, t36: F, t1083: F, t6764: F, t1080: F, t2389: F, t2911: F) -> (F, F, F, F, F, F) {
    let t17140 = t350 * t6821;
    let t17143 = t1464 * t5974 * t337;
    let t17145 = t36 * t1476 * t17143;
    let t17147 = t6764 * t1083;
    let t17149 = t36 * t1476 * t17147;
    let t17152 = t2911 * t2389 * t1080;
    (t17140, t17143, t17145, t17147, t17149, t17152)
}
