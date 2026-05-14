//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 849/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk849<F: Float>(t2623: F, t497: F, t337: F, t1380: F, t1907: F, t5482: F, t1924: F, t5486: F, t2542: F, t350: F, t2909: F, t6508: F, t36: F, t1476: F, t6503: F, t6512: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6781 = t2623 * t497;
    let t6782 = t6781 * t337;
    let t6783 = t1380 * t6782;
    let t6788 = t5482 * t1907;
    let t6791 = t5486 * t1924;
    let t6800 = t350 * t2542;
    let t6802 = t2909 * t6508;
    let t6803 = t36 * t6802;
    let t6805 = t1476 * t6503;
    let t6806 = t36 * t6805;
    let t6808 = t1476 * t6512;
    (t6781, t6782, t6783, t6788, t6791, t6800, t6802, t6803, t6805, t6806, t6808)
}
