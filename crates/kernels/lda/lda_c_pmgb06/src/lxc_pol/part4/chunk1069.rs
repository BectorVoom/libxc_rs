//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1069/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1069<F: Float>(t2496: F, t493: F, t9925: F, t2979: F, t6390: F, t10152: F, t6517: F, t1908: F, t5187: F, t1420: F, t6524: F, t10288: F, t439: F, t6523: F, t1444: F, t6518: F) -> (F, F, F, F, F, F, F) {
    let t15965 = 2.0 / 45.0 * t493 * t9925 * t2496;
    let t15968 = 4.0 / 45.0 * t493 * t2979 * t6390;
    let t15971 = 4.0 / 45.0 * t493 * t10152 * t6517;
    let t15973 = 4.0 / 45.0 * t5187 * t1908;
    let t15975 = 4.0 / 45.0 * t1420 * t6524;
    let t15978 = 4.0 / 45.0 * t439 * t10288 * t6523;
    let t15980 = 4.0 / 45.0 * t1444 * t6518;
    (t15965, t15968, t15971, t15973, t15975, t15978, t15980)
}
