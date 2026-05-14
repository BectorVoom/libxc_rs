//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 940/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk940<F: Float>(t1436: F, t1439: F, t2010: F, t332: F, t1423: F, t4767: F, t1558: F, t442: F, t130: F, t431: F, t5076: F, t5079: F, t1386: F, t1593: F, t2064: F, t5077: F) -> (F, F, F, F, F, F, F) {
    let t12676 = 2.0 / 9.0 * t2010 * t1436 * t1439 * t332;
    let t12677 = t1423 * t4767;
    let t12678 = 2.0 / 5.0 * t12677;
    let t12682 = 4.0 / 15.0 * t2010 * t442 * t1558 * t332;
    let t12683 = t431 * t130;
    let t12684 = t12683 * t5076;
    let t12686 = 4.0 / 15.0 * t12684 * t5079;
    let t12690 = 4.0 / 15.0 * t5077 * t1593 * t2064 * t1386;
    (t12676, t12678, t12682, t12683, t12684, t12686, t12690)
}
