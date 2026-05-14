//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1164/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1164<F: Float>(t1555: F, t2563: F, t2064: F, t132: F, t137: F, t1593: F, t1558: F, t2010: F, t442: F, t760: F, t13432: F, t1423: F, t6297: F, t129: F, t15844: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t17563 = t2563 * t1555;
    let t17564 = t17563 / 135.0;
    let t17567 = t2064 * t2064;
    let t17571 = 2.0 / 15.0 * t132 * t137 * t1593 * t17567;
    let t17575 = 8.0 / 45.0 * t2010 * t442 * t1558 * t760;
    let t17576 = 4.0 / 45.0 * t13432;
    let t17577 = t1423 * t6297;
    let t17578 = 8.0 / 135.0 * t17577;
    let t17579 = t129 * t15844;
    let t17583 = 16.0 / 45.0 * t17579 * t442 * t1558 * t79;
    (t17564, t17571, t17575, t17576, t17578, t17579, t17583)
}
