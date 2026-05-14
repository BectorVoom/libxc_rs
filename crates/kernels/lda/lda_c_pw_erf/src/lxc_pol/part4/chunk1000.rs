//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1000/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1000<F: Float>(t1390: F, t1458: F, t1245: F, t108: F, t267: F, t510: F, t12118: F, t4491: F, t4561: F, t565: F, t3704: F, t4487: F) -> (F, F, F, F, F) {
    let t12321 = t1458 * t1390;
    let t12322 = t12321 * t1245;
    let t12329 = t510 * t108 * t267;
    let t12338 = t12118 * t4491;
    let t12356 = t565 * t4561;
    let t12362 = t4487 * t3704;
    (t12322, t12329, t12338, t12356, t12362)
}
