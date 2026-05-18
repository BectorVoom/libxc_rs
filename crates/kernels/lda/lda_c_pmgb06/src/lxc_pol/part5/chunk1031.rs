//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1031/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1031<F: Float>(t1: F, t2541: F, t1830: F, t506: F, t1825: F, t5974: F, t36: F, t2389: F, t4851: F, t1414: F, t337: F, t7300: F) -> (F, F, F, F, F, F, F) {
    let t19336 = t2541 * t1;
    let t19338 = t1830 * t506 * t19336;
    let t19340 = t1825 * t5974;
    let t19342 = t36 * t506 * t19340;
    let t19344 = t4851 * t2389;
    let t19346 = t1830 * t506 * t19344;
    let t19349 = t1414 * t7300 * t337;
    (t19336, t19338, t19340, t19342, t19344, t19346, t19349)
}
