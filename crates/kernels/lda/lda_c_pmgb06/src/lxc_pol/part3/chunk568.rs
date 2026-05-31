//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 568/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk568<F: Float>(t177: F, t3004: F, t161: F, t1423: F, t1560: F, t1069: F, t332: F) -> (F, F, F, F, F) {
    let t3005 = t3004 * t177;
    let t3007 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t161 * t3005;
    let t3008 = t1423 * t1560;
    let t3009 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t3008;
    let t3010 = t1069 * t332;
    (t3005, t3007, t3008, t3009, t3010)
}
