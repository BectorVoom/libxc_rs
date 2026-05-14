//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 593/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk593<F: Float>(t1282: F, t3588: F, t35: F, t27: F, t365: F, t109: F, t1234: F, t55: F, t348: F, t64: F, t1243: F, t3582: F, t3559: F, t38: F, t56: F, t370: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3589 = t1282 * t3588;
    let t3590 = t35 * t3589;
    let t3594 = t365 * t1282 * t27;
    let t3596 = t55 * t109 * t1234;
    let t3597 = t3594 * t3596;
    let t3600 = t348 * t64 * t27;
    let t3601 = t3600 * t3596;
    let t3602 = 2.923025 * t3601;
    let t3603 = t1243 * t3582;
    let t3604 = 1.9486833333333333 * t3603;
    let t3607 = 2.923025 * t38 * t56 * t3559;
    let t3608 = t370 * t3559;
    (t3589, t3590, t3594, t3597, t3600, t3601, t3602, t3603, t3604, t3607, t3608)
}
