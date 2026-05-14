//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1141/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1141<F: Float>(t12196: F, t12251: F, t12297: F, t12307: F, t12309: F, t12311: F, t1982: F, t2100: F, t1298: F, t6592: F, t12475: F, t16031: F, t3967: F, t494: F, t12362: F, t16032: F, t6710: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16785 = 32.0 / 405.0 * t12196;
    let t16786 = 64.0 / 135.0 * t12251;
    let t16787 = 32.0 / 135.0 * t12297;
    let t16788 = 32.0 / 45.0 * t12307;
    let t16789 = 16.0 / 135.0 * t12309;
    let t16790 = 16.0 / 45.0 * t12311;
    let t16792 = 8.0 / 15.0 * t1982 * t2100;
    let t16794 = 8.0 / 15.0 * t1298 * t6592;
    let t16798 = 64.0 / 45.0 * t12475 * t3967 * t16031 * t494;
    let t16801 = 64.0 / 45.0 * t12362 * t6710 * t16032;
    (t16785, t16786, t16787, t16788, t16789, t16790, t16792, t16794, t16798, t16801)
}
