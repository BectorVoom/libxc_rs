//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 382/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk382<F: Float>(t1525: F, t1859: F, t36: F, t1531: F, t760: F, t332: F) -> (F, F, F, F) {
    let t1860 = t1525 * t1859;
    let t1861 = t36 * t1860;
    let t1863 = t1531 * t760;
    let t1864 = t1863 * t332;
    (t1860, t1861, t1863, t1864)
}
