//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 989/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk989<F: Float>(t16563: F, t1893: F, t5077: F, t1864: F, t2649: F, t1859: F, t5083: F, t15855: F, t6639: F, t6643: F, t15858: F, t6646: F, t17719: F, t1911: F, t5068: F, t1826: F, t2624: F) -> (F, F, F, F, F, F, F, F) {
    let t20533 = 2.0 / 15.0 * t5077 * t16563 * t1893;
    let t20536 = 2.0 / 15.0 * t5077 * t2649 * t1864;
    let t20539 = t5083 * t2649 * t1859 / 9.0;
    let t20541 = 4.0 / 15.0 * t15855 * t6639;
    let t20543 = 4.0 / 15.0 * t15855 * t6643;
    let t20545 = 2.0 / 9.0 * t15858 * t6646;
    let t20548 = 2.0 / 15.0 * t5068 * t17719 * t1911;
    let t20551 = 2.0 / 15.0 * t5068 * t2624 * t1826;
    (t20533, t20536, t20539, t20541, t20543, t20545, t20548, t20551)
}
