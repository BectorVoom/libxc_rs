//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 986/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk986<F: Float>(t1848: F, t2654: F, t6461: F, t831: F, t20478: F, t20480: F, t20482: F, t20486: F, t20490: F, t20491: F, t20492: F, t20493: F, t20495: F, t16877: F, t6731: F, t16880: F) -> (F, F, F, F, F, F) {
    let t20497 = t1848 * t2654 / 5.0;
    let t20499 = t831 * t6461 / 5.0;
    let t20500 = t20478 - t20480 + t20482 - t20486 + t20490 - t20491 - t20492 - t20493 - t20495 - t20497 - t20499;
    let t20501 = t16877 / 15.0;
    let t20503 = t831 * t6731 / 5.0;
    let t20504 = 2.0 / 45.0 * t16880;
    (t20497, t20499, t20500, t20501, t20503, t20504)
}
