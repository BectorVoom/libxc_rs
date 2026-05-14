//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 825/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk825<F: Float>(t1350: F, t1475: F, t3751: F, t571: F, t1318: F, t3899: F, t3904: F, t3756: F, t155: F, t213: F, t1468: F, t2151: F, t576: F, t352: F, t954: F, t348: F, t945: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9409 = t1350 * t1350;
    let t9410 = 1.0 / t9409;
    let t9424 = t571 * t1475 * t3751;
    let t9427 = t1318 * t3899 * t3904;
    let t9430 = t571 * t3899 * t3756;
    let t9432 = t155 * t213;
    let t9434 = t1318 * t9432 * t1468;
    let t9436 = t2151 * t576;
    let t9437 = t571 * t9436;
    let t9456 = t954 * t352;
    let t9481 = t348 * t945;
    (t9410, t9424, t9427, t9430, t9432, t9434, t9436, t9437, t9456, t9481)
}
