//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 662/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk662<F: Float>(t3568: F, t3576: F, t1243: F, t3572: F, t342: F, t409: F, t55: F, t1276: F, t110: F, t1263: F, t360: F, t1282: F, t27: F, t365: F) -> (F, F, F, F, F, F, F) {
    let t3577 = t3576 * t3568;
    let t3579 = t1243 * t3572;
    let t3582 = t55 * t409 * t342;
    let t3583 = t1276 * t3582;
    let t3585 = t110 * t1263;
    let t3586 = t360 * t3585;
    let t3594 = t365 * t1282 * t27;
    (t3577, t3579, t3582, t3583, t3585, t3586, t3594)
}
