//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1093/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1093<F: Float>(t19645: F, t19647: F, t19650: F, t10: F, t128: F, t20283: F, t325: F, t431: F, t7930: F, t415: F, t7933: F, t7924: F) -> (F, F, F, F, F, F, F) {
    let t20340 = F::cast_from(8.769075_f64) * t19645;
    let t20341 = F::cast_from(5.84605_f64) * t19647;
    let t20342 = F::cast_from(2.923025_f64) * t19650;
    let t20345 = t10 * t128 * t20283;
    let t20349 = t431 * t7930 * t325;
    let t20352 = t415 * t7933 * t325;
    let t20353 = F::cast_from(2.923025_f64) * t20352;
    let t20355 = t415 * t7924 * t325;
    (t20340, t20341, t20342, t20345, t20349, t20353, t20355)
}
