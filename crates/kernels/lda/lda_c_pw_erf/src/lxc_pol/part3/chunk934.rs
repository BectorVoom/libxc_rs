//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 934/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk934<F: Float>(t1278: F, t348: F, t739: F, t4488: F, t4494: F, t12118: F, t4491: F, t12329: F, t1314: F, t2098: F, t4489: F, t3846: F, t4490: F, t3850: F, t12321: F, t3403: F, t806: F) -> (F, F, F, F, F, F, F, F) {
    let t12334 = t739 * t1278 * t348;
    let t12337 = 8.0 / 15.0 * t4488 * t4494 * t12334;
    let t12338 = t12118 * t4491;
    let t12339 = 32.0 / 45.0 * t12338;
    let t12341 = 16.0 / 15.0 * t12329 * t4491;
    let t12345 = 16.0 / 15.0 * t4488 * t4489 * t2098 * t1314;
    let t12348 = 8.0 / 15.0 * t4488 * t4490 * t3846;
    let t12351 = 8.0 / 15.0 * t4488 * t4490 * t3850;
    let t12355 = 8.0 / 9.0 * t4488 * t12321 * t806 * t3403;
    (t12334, t12337, t12339, t12341, t12345, t12348, t12351, t12355)
}
