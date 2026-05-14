//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1162/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1162<F: Float>(t439: F, t5197: F, t6258: F, t1444: F, t6287: F, t6528: F, t6254: F, t2623: F, t3457: F, t1602: F, t1992: F, t493: F, t27: F, t545: F, t7209: F, t7179: F) -> (F, F, F, F, F, F, F) {
    let t17530 = 4.0 / 15.0 * t439 * t5197 * t6258;
    let t17532 = 2.0 / 5.0 * t1444 * t6287;
    let t17534 = 4.0 / 15.0 * t1444 * t6528;
    let t17537 = 2.0 / 5.0 * t439 * t5197 * t6254;
    let t17538 = t3457 * t2623;
    let t17542 = t493 * t1992 * t17538 * t1602 / 5.0;
    let t17544 = t7209 * t27 * t545;
    let t17547 = t7179 * t27 * t545;
    (t17530, t17532, t17534, t17537, t17542, t17544, t17547)
}
