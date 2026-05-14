//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1158/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1158<F: Float>(t1446: F, t6909: F, t3416: F, t6965: F, t1280: F, t2407: F, t2114: F, t6220: F, t1298: F, t493: F, t514: F, t6591: F, t544: F, t6788: F, t14110: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t17053 = 16.0 / 15.0 * t1446 * t6909;
    let t17055 = 16.0 / 15.0 * t3416 * t6965;
    let t17057 = 4.0 / 15.0 * t2407 * t1280;
    let t17058 = t2114 * t6220;
    let t17059 = 16.0 / 45.0 * t17058;
    let t17060 = t1298 * t6220;
    let t17061 = 16.0 / 45.0 * t17060;
    let t17063 = t493 * t514 * t6591;
    let t17064 = 16.0 / 45.0 * t17063;
    let t17066 = 8.0 / 15.0 * t6788 * t544;
    let t17069 = 8.0 / 15.0 * t14110 * t786;
    (t17053, t17055, t17057, t17059, t17061, t17064, t17066, t17069)
}
