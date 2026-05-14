//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1114/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1114<F: Float>(t1310: F, t6205: F, t2146: F, t4770: F, t12083: F, t9380: F, t12087: F, t6378: F, t951: F, t11: F, t557: F, t6365: F, t5021: F, t6815: F, t331: F, t6812: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16266 = 8.0 / 45.0 * t6205 * t1310;
    let t16268 = 16.0 / 45.0 * t2146 * t4770;
    let t16269 = 32.0 / 405.0 * t12083;
    let t16270 = 4.0 / 135.0 * t9380;
    let t16271 = 32.0 / 45.0 * t12087;
    let t16274 = t6378 * t951;
    let t16276 = t11 * t557 * t16274;
    let t16278 = t6365 * t951;
    let t16280 = t11 * t557 * t16278;
    let t16285 = t5021 * t6815;
    let t16287 = t331 * t6812;
    (t16266, t16268, t16269, t16270, t16271, t16274, t16276, t16278, t16280, t16285, t16287)
}
