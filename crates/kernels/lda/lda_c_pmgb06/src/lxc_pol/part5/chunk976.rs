//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 976/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk976<F: Float>(t439: F, t6254: F, t6550: F, t6258: F, t20323: F, t20324: F, t20325: F, t20328: F, t20330: F, t20332: F, t20334: F, t20337: F, t20338: F, t20340: F, t17964: F, t1992: F, t493: F, t851: F) -> (F, F, F, F) {
    let t20343 = 3.0 / 5.0 * t439 * t6550 * t6254;
    let t20346 = 2.0 / 5.0 * t439 * t6550 * t6258;
    let t20347 = t20323 + t20324 + t20325 - t20328 - t20330 - t20332 - t20334 - t20337 + t20338 + t20340 - t20343 + t20346;
    let t20353 = t493 * t1992 * t17964 * t851 / 5.0;
    (t20343, t20346, t20347, t20353)
}
