//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 809/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk809<F: Float>(t4093: F, t588: F, t97: F, t208: F, t213: F, t2803: F, t579: F, t4083: F, t138: F, t163: F, t9175: F, t2925: F, t350: F, t2934: F, t139: F, t3247: F) -> (F, F, F, F, F, F, F, F) {
    let t9483 = t4093 * t97 * t588;
    let t9491 = t2803 * t579 * t208 * t213;
    let t9494 = t4083 * t97 * t588;
    let t9501 = t138 * t9175 * t163;
    let t9502 = 0.01959135802469136 * t9501;
    let t9503 = t350 * t2925;
    let t9505 = t350 * t2934;
    let t9507 = t139 * t3247;
    (t9483, t9491, t9494, t9501, t9502, t9503, t9505, t9507)
}
