//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 783/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk783<F: Float>(t2157: F, t3222: F, t1185: F, t346: F, t825: F, t38: F, t368: F, t4340: F, t4348: F, t4498: F, t4502: F, t4505: F, t4512: F, t4538: F, t4541: F, t4744: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11540 = t2157 * param_a_c;
    let t11541 = t11540 * t3222;
    let t12076 = t346 * t825 * t1185;
    let t15651 = t38 * t38;
    let t15652 = 1.0 / t15651;
    let t16191 = t368 * t368;
    let t16192 = 1.0 / t16191;
    let t16329 = 0.12654485932329694421e2 * t4340;
    let t16331 = 0.73024584604562962965e1 * t4348;
    let t16334 = 48.0 * t4498;
    let t16335 = 0.19298189186581325787e3 * t4502;
    let t16336 = 24.0 * t4505;
    let t16337 = 0.38596378373162651572e3 * t4512;
    let t16338 = 4.0 * t4538;
    let t16340 = 24.0 * t4541;
    let t16345 = 4.0 * t4744;
    (t11541, t12076, t15651, t15652, t16192, t16329, t16331, t16334, t16335, t16336, t16337, t16338, t16340, t16345)
}
