//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 964/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk964<F: Float>(t20147: F, t337: F, t12529: F, t12530: F, t1: F, t6560: F, t12537: F, t5139: F, t2018: F, t2563: F, t16558: F, t439: F, t5482: F, t6412: F, t6160: F, t6494: F) -> (F, F, F, F, F, F, F, F) {
    let t20152 = t20147 * t337;
    let t20155 = 8.0 / 27.0 * t12529 * t12530 * t20152;
    let t20156 = t6560 * t1;
    let t20159 = 4.0 / 9.0 * t12537 * t5139 * t20156;
    let t20160 = t2563 * t2018;
    let t20161 = t20160 / 15.0;
    let t20162 = t16558 / 15.0;
    let t20165 = t439 * t5482 * t6412 / 15.0;
    let t20168 = 2.0 / 15.0 * t439 * t6494 * t6160;
    (t20152, t20155, t20156, t20159, t20161, t20162, t20165, t20168)
}
