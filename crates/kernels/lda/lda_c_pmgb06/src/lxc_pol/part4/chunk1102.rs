//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1102/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1102<F: Float>(t439: F, t4672: F, t6494: F, t4650: F, t6498: F, t2010: F, t4668: F, t1420: F, t6499: F, t12092: F, t153: F, t1859: F, t4659: F, t13715: F, t4645: F, t4655: F) -> (F, F, F, F, F, F, F, F) {
    let t16475 = 4.0 / 45.0 * t439 * t6494 * t4672;
    let t16478 = 4.0 / 9.0 * t439 * t6498 * t4650;
    let t16481 = 16.0 / 45.0 * t2010 * t6494 * t4668;
    let t16483 = 4.0 / 27.0 * t1420 * t6499;
    let t16487 = 4.0 / 27.0 * t439 * t12092 * t153 * t1859;
    let t16490 = 2.0 / 27.0 * t439 * t6498 * t4659;
    let t16491 = t13715 * t153;
    let t16494 = 16.0 / 81.0 * t439 * t16491 * t4645;
    let t16497 = 8.0 / 27.0 * t2010 * t6498 * t4655;
    (t16475, t16478, t16481, t16483, t16487, t16490, t16494, t16497)
}
