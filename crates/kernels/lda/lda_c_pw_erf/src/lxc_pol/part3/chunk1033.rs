//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1033/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1033<F: Float>(t9392: F, t9424: F, t9427: F, t9430: F, t9434: F, t9437: F, t3610: F, t3974: F, t6752: F, t4500: F, t806: F, t3482: F, t4488: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12100 = F::new(8.0) / F::new(135.0) * t9392;
    let t12101 = F::new(16.0) / F::new(45.0) * t9424;
    let t12102 = F::new(8.0) / F::new(15.0) * t9427;
    let t12103 = F::new(8.0) / F::new(15.0) * t9430;
    let t12104 = F::new(16.0) / F::new(45.0) * t9434;
    let t12105 = F::new(32.0) / F::new(405.0) * t9437;
    let t12108 = F::new(8.0) / F::new(9.0) * t3974 * t6752 * t3610;
    let t12109 = t4500 * t806;
    let t12112 = F::new(4.0) / F::new(9.0) * t4488 * t12109 * t3482;
    (t12100, t12101, t12102, t12103, t12104, t12105, t12108, t12109, t12112)
}
