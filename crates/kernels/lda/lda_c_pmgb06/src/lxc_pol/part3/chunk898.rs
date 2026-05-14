//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 898/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk898<F: Float>(t1594: F, t1868: F, t2010: F, t2864: F, t12000: F, t12003: F, t12005: F, t12011: F, t12015: F, t12017: F, t12019: F, t12021: F, t12023: F, t12026: F, t12028: F, t1420: F, t5350: F) -> (F, F, F) {
    let t12032 = 4.0 / 15.0 * t2010 * t2864 * t1868 * t1594;
    let t12033 = -t12000 + t12003 - t12005 - t12011 - t12015 - t12017 - t12019 - t12021 + t12023 + t12026 + t12028 + t12032;
    let t12035 = 2.0 / 5.0 * t1420 * t5350;
    (t12032, t12033, t12035)
}
