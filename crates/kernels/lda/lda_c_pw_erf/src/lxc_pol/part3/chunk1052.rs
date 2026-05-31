//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1052/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1052<F: Float>(t1392: F, t739: F, t348: F, t12322: F, t4488: F, t108: F, t267: F, t510: F, t4497: F, t12292: F, t12294: F, t12296: F, t12298: F, t12301: F, t12305: F, t12308: F, t12310: F, t12312: F, t12316: F, t12320: F) -> (F, F, F, F, F, F) {
    let t12323 = t739 * t1392;
    let t12324 = t12323 * t348;
    let t12327 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4488 * t12322 * t12324;
    let t12329 = t510 * t108 * t267;
    let t12331 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12329 * t4497;
    let t12332 = t12292 + t12294 - t12296 + t12298 - t12301 - t12305 + t12308 + t12310 - t12312 - t12316 + t12320 + t12327 + t12331;
    (t12323, t12324, t12327, t12329, t12331, t12332)
}
