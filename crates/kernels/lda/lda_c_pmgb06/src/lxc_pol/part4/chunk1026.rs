//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1026/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1026<F: Float>(t1423: F, t6524: F, t1069: F, t1531: F, t2604: F, t2864: F, t439: F, t1080: F, t6502: F, t1915: F, t493: F, t1602: F, t2545: F, t2871: F, t12633: F, t5364: F) -> (F, F, F, F, F, F) {
    let t15216 = t1423 * t6524;
    let t15217 = 8.0 / 135.0 * t15216;
    let t15222 = 4.0 / 45.0 * t439 * t2864 * t2604 * t1531 * t1069;
    let t15223 = t6502 * t1080;
    let t15226 = 8.0 / 15.0 * t493 * t1915 * t15223;
    let t15230 = 4.0 / 45.0 * t493 * t2871 * t2545 * t1602;
    let t15233 = 4.0 / 45.0 * t439 * t12633 * t5364;
    (t15217, t15222, t15223, t15226, t15230, t15233)
}
