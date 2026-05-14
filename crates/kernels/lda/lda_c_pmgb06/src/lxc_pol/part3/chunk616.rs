//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 616/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk616<F: Float>(t285: F, t3933: F, t248: F, t3874: F, t3877: F, t3881: F, t3884: F, t3888: F, t3899: F, t3901: F, t3904: F, t3906: F, t3908: F, t3911: F, t1105: F, t654: F) -> (F, F, F) {
    let t3934 = t3933 * t285;
    let t3936 = t248 * t3934 + t3874 + t3877 + t3881 - t3884 - t3888 - 12.0 * t3899 + 24.0 * t3901 + 3.0 * t3904 - 96.0 * t3906 + 60.0 * t3908 + t3911;
    let t3939 = t1105 * t654;
    (t3934, t3936, t3939)
}
