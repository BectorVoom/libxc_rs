//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 627/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk627<F: Float>(t22: F, t4048: F, t219: F, t3589: F, t2967: F, t571: F, t1472: F, t1480: F, t1488: F, t1475: F, t1479: F, t1484: F, t9: F, t1487: F, t2973: F, t575: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4049 = t22 * t4048;
    let t4050 = t219 * t3589;
    let t4051 = t4050 * t2967;
    let t4052 = t4049 * t4051;
    let t4054 = 32.0 / 81.0 * t571 * t4052;
    let t4056 = 4.0 / 15.0 * t1472 * t1480;
    let t4058 = 4.0 / 9.0 * t1472 * t1488;
    let t4059 = t1475 * t1479;
    let t4060 = t571 * t4059;
    let t4061 = 8.0 / 45.0 * t4060;
    let t4062 = t9 * t1484;
    let t4063 = t4062 * t1487;
    let t4064 = t571 * t4063;
    let t4065 = 8.0 / 27.0 * t4064;
    let t4066 = t575 * t2973;
    (t4049, t4051, t4052, t4054, t4056, t4058, t4059, t4060, t4061, t4062, t4063, t4064, t4065, t4066)
}
