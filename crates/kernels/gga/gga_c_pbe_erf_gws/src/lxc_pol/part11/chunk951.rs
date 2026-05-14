//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 951/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk951<F: Float>(t3047: F, t9955: F, t13112: F, t22493: F, t13650: F, t4414: F, t13700: F, t2053: F, t3703: F, t10424: F, t1820: F, t1821: F, t3346: F, t2559: F, t30455: F, t3342: F) -> (F, F, F, F, F, F, F) {
    let t47084 = t9955 * t3047;
    let t47087 = t22493 * t13112;
    let t47143 = t4414 * t13650;
    let t47169 = t13700 * t2053;
    let t47181 = param_gamma * t3703;
    let t47293 = 16.0 / 15.0 * t1820 * t1821 * t10424 * t3346;
    let t47297 = 16.0 / 9.0 * t1820 * t2559 * t30455 * t3342;
    (t47084, t47087, t47143, t47169, t47181, t47293, t47297)
}
