//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 938/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk938<F: Float>(t12649: F, t4761: F, t493: F, t5179: F, t9596: F, t9598: F, t9601: F, t1980: F, t883: F, t4713: F, t607: F, t1710: F, t1959: F, t432: F, t4979: F, t9616: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12650 = 2.0 / 135.0 * t12649;
    let t12653 = 3.0 / 5.0 * t493 * t5179 * t4761;
    let t12654 = 4.0 / 135.0 * t9596;
    let t12655 = 2.0 / 45.0 * t9598;
    let t12656 = 2.0 / 45.0 * t9601;
    let t12657 = t883 * t1980;
    let t12659 = t4713 * t607;
    let t12661 = t1959 * t1710;
    let t12662 = 2.0 / 45.0 * t12661;
    let t12664 = t432 * t4979 / 10.0;
    let t12665 = t9616 / 15.0;
    (t12650, t12653, t12654, t12655, t12656, t12657, t12659, t12662, t12664, t12665)
}
