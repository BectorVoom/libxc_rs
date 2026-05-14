//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1102/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1102<F: Float>(t2388: F, t571: F, t9313: F, t1392: F, t2429: F, t519: F, t9504: F, t1518: F, t185: F, t2472: F, t12065: F, t6374: F, t9278: F, t14240: F, t6384: F, t13455: F, t6388: F) -> (F, F, F, F, F, F, F) {
    let t16058 = t571 * t9313 * t2388;
    let t16059 = 16.0 / 405.0 * t16058;
    let t16063 = 8.0 / 27.0 * t519 * t9504 * t2429 * t1392;
    let t16065 = t185 * t1518 * t2472;
    let t16066 = 8.0 / 135.0 * t16065;
    let t16067 = 64.0 / 135.0 * t12065;
    let t16069 = t571 * t9278 * t6374;
    let t16070 = 16.0 / 81.0 * t16069;
    let t16072 = t571 * t14240 * t6384;
    let t16073 = 128.0 / 243.0 * t16072;
    let t16075 = t571 * t13455 * t6388;
    (t16059, t16063, t16066, t16067, t16070, t16073, t16075)
}
