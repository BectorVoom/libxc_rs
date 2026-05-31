//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1195/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1195<F: Float>(t1423: F, t6379: F, t6472: F, t5211: F, t6382: F, t1420: F, t6376: F, t2948: F, t439: F, t6375: F, t1385: F, t1629: F, t2574: F) -> (F, F, F, F, F, F) {
    let t15770 = t1423 * t6379;
    let t15771 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t15770;
    let t15772 = t1423 * t6472;
    let t15773 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t15772;
    let t15774 = t5211 * t6382;
    let t15775 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t15774;
    let t15777 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1420 * t6376;
    let t15780 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t2948 * t6375;
    let t15784 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t1385 * t2574 * t1629;
    (t15771, t15773, t15775, t15777, t15780, t15784)
}
