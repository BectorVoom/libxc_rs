//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 844/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk844<F: Float>(t571: F, t6380: F, t2334: F, t3589: F, t352: F, t4776: F, t1943: F, t34: F, t4868: F, t2027: F, t4738: F, t1472: F, t2389: F, t2065: F, t816: F, t1308: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6382 = 8.0 / 9.0 * t571 * t6380;
    let t6383 = t3589 * t2334;
    let t6384 = t6383 * t352;
    let t6385 = t4776 * t6384;
    let t6387 = 32.0 / 81.0 * t571 * t6385;
    let t6388 = t1943 * t34;
    let t6389 = t4868 * t6388;
    let t6391 = 16.0 / 27.0 * t571 * t6389;
    let t6393 = 16.0 / 45.0 * t4738 * t2027;
    let t6395 = 8.0 / 45.0 * t1472 * t2389;
    let t6396 = t816 * t2065;
    let t6397 = t1308 * t6396;
    (t6382, t6383, t6384, t6385, t6387, t6388, t6389, t6391, t6393, t6395, t6396, t6397)
}
