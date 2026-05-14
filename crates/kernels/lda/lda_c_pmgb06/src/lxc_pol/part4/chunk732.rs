//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 732/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk732<F: Float>(t500: F, t5194: F, t136: F, t458: F, t1968: F, t439: F, t1592: F, t2064: F, t477: F, t1966: F, t3220: F, t806: F, t1423: F, t2007: F, t1179: F, t131: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5196 = 4.0 / 135.0 * t5194 * t500;
    let t5197 = t136 * t458;
    let t5198 = t5197 * t1968;
    let t5200 = 2.0 / 15.0 * t439 * t5198;
    let t5201 = t1592 * t2064;
    let t5202 = t5201 * t477;
    let t5203 = t1966 * t5202;
    let t5205 = 2.0 / 15.0 * t439 * t5203;
    let t5207 = 4.0 / 135.0 * t3220 * t806;
    let t5209 = 4.0 / 135.0 * t1423 * t2007;
    let t5210 = t131 * t1179;
    (t5196, t5197, t5198, t5200, t5202, t5203, t5205, t5207, t5209, t5210)
}
