//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1070/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1070<F: Float>(t1925: F, t3226: F, t1600: F, t1988: F, t1898: F, t3213: F, t161: F, t3004: F, t843: F, t132: F, t1547: F, t2065: F) -> (F, F, F, F, F) {
    let t11875 = t3226 * t1925;
    let t11877 = t1988 * t1600;
    let t11881 = t3213 * t1898;
    let t11884 = t161 * t3004 * t843;
    let t11897 = t132 * t1547 * t2065;
    (t11875, t11877, t11881, t11884, t11897)
}
