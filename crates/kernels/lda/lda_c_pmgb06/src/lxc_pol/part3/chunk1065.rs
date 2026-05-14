//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1065/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1065<F: Float>(t1409: F, t1798: F, t188: F, t3023: F, t794: F, t11589: F, t14005: F, t14007: F, t14010: F, t14012: F, t14014: F, t14016: F, t14018: F, t14020: F, t14478: F, t183: F) -> (F,) {
    let t14481 = t1798 * t1409 * t188;
    let t14482 = 4.0 * t14481;
    let t14484 = t794 * t3023 * t188;
    let t14486 = -t14005 - t14007 - t14010 - t14012 - t14014 + t14016 - t14018 - t14020 + 4.0 / 3.0 * t11589 * t183 * t188 + 4.0 * t14478 + t14482 + 4.0 / 3.0 * t14484;
    (t14486,)
}
