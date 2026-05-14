//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 546/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk546<F: Float>(t1325: F, t2393: F, t789: F, t806: F, t1313: F, t519: F, t811: F, t209: F, t184: F) -> (F, F, F, F, F, F, F) {
    let t2395 = 16.0 / 45.0 * t1325 * t2393;
    let t2396 = t789 * t806;
    let t2397 = t1313 * t2396;
    let t2399 = 8.0 / 45.0 * t519 * t2397;
    let t2400 = t811 * t811;
    let t2401 = t2400 * t209;
    let t2402 = t2401 * t184;
    (t2395, t2396, t2397, t2399, t2400, t2401, t2402)
}
