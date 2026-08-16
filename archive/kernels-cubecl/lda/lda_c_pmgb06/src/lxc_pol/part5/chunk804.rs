//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 804/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk804<F: Float>(t1901: F, t7485: F, t439: F, t2570: F, t822: F, t2960: F, t2578: F, t1385: F, t6516: F, t764: F, t2871: F, t493: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7547 = t1901 * t7485;
    let t7549 = t439 * t7547 / F::cast_from(9.0_f64);
    let t7550 = t2570 * t822;
    let t7551 = t2960 * t7550;
    let t7553 = t439 * t7551 / F::cast_from(9.0_f64);
    let t7554 = t2578 * t822;
    let t7555 = t1385 * t7554;
    let t7557 = t439 * t7555 / F::cast_from(15.0_f64);
    let t7558 = t6516 * t764;
    let t7559 = t2871 * t7558;
    let t7561 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t7559;
    (t7547, t7549, t7550, t7551, t7553, t7554, t7555, t7557, t7558, t7559, t7561)
}
