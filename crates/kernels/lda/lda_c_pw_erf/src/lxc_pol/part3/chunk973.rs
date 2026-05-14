//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 973/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk973<F: Float>(t12951: F, t3872: F, t3974: F, t4475: F, t4489: F, t784: F, t3807: F, t3965: F, t3811: F, t4479: F, t34: F, t3966: F, t12475: F, t1314: F, t12937: F, t12941: F, t12943: F, t12945: F, t12947: F, t12948: F, t12949: F, t12950: F) -> (F, F, F, F, F, F) {
    let t12952 = 4.0 / 15.0 * t12951;
    let t12955 = 16.0 / 15.0 * t3974 * t4475 * t3872;
    let t12956 = t4489 * t784;
    let t12959 = 16.0 / 15.0 * t3965 * t12956 * t3807;
    let t12962 = 16.0 / 15.0 * t3965 * t4479 * t3811;
    let t12963 = t3966 * t34;
    let t12966 = 16.0 / 15.0 * t12475 * t12963 * t1314;
    let t12967 = -t12937 + t12941 + t12943 + t12945 + t12947 + t12948 + t12949 + t12950 + t12952 + t12955 + t12959 + t12962 - t12966;
    (t12952, t12955, t12959, t12962, t12966, t12967)
}
