//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 919/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk919<F: Float>(t9427: F, t9430: F, t9434: F, t9437: F, t3610: F, t3974: F, t6752: F, t4500: F, t806: F, t3482: F, t4488: F, t1245: F, t3966: F, t4495: F, t940: F, t4487: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12102 = 8.0 / 15.0 * t9427;
    let t12103 = 8.0 / 15.0 * t9430;
    let t12104 = 16.0 / 45.0 * t9434;
    let t12105 = 32.0 / 405.0 * t9437;
    let t12108 = 8.0 / 9.0 * t3974 * t6752 * t3610;
    let t12109 = t4500 * t806;
    let t12112 = 4.0 / 9.0 * t4488 * t12109 * t3482;
    let t12113 = t3966 * t1245;
    let t12114 = t4495 * t940;
    let t12117 = 8.0 / 5.0 * t4488 * t12113 * t12114;
    let t12118 = t4487 * t668;
    (t12102, t12103, t12104, t12105, t12108, t12109, t12112, t12114, t12117, t12118)
}
