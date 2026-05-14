//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 917/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk917<F: Float>(t12396: F, t13392: F, t19314: F, t13388: F, t13384: F, t350: F, t7606: F, t7613: F, t337: F, t7598: F, t36: F, t506: F, t1: F, t2541: F, t1830: F, t1825: F, t5974: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19316 = t12396 * t13392 * t19314;
    let t19319 = t12396 * t13388 * t19314;
    let t19322 = t12396 * t13384 * t19314;
    let t19324 = t350 * t7606;
    let t19326 = t350 * t7613;
    let t19332 = t7598 * t337;
    let t19334 = t36 * t506 * t19332;
    let t19336 = t2541 * t1;
    let t19338 = t1830 * t506 * t19336;
    let t19340 = t1825 * t5974;
    (t19316, t19319, t19322, t19324, t19326, t19332, t19334, t19336, t19338, t19340)
}
