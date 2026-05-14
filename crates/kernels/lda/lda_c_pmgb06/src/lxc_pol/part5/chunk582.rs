//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 582/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk582<F: Float>(t248: F, t4481: F, t2158: F, t643: F, t3912: F, t760: F, t1: F, t1068: F, t3922: F, t764: F, t1079: F, t2160: F, t638: F, t1105: F, t898: F, t1101: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4483 = 2.0 * t248 * t4481;
    let t4485 = 8.0 * t643 * t2158;
    let t4486 = t3912 * t760;
    let t4489 = t1068 * t1;
    let t4500 = t3922 * t764;
    let t4503 = t1079 * t1;
    let t4518 = t638 * t2160;
    let t4520 = t1105 * t898;
    let t4522 = t1101 * t898;
    (t4483, t4485, t4486, t4489, t4500, t4503, t4518, t4520, t4522)
}
