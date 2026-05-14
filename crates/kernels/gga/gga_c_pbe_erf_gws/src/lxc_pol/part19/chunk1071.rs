//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1071/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1071<F: Float>(t44205: F, t9607: F, t3222: F, t39052: F, t37632: F, t3327: F, t810: F, t332: F, t4395: F, t2157: F, t938: F, t2271: F, t824: F, t838: F, t822: F, t2331: F, t328: F, t356: F, t3971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44206 = t9607 * t44205;
    let t45096 = t39052 * t3222;
    let t46392 = t37632 * t3222;
    let t47184 = t3327 * t810;
    let t50887 = t4395 * t332;
    let t50912 = t2157 * t938;
    let t50935 = t2271 * t332;
    let t50942 = t824 * t838;
    let t50943 = t822 * t50942;
    let t50948 = t356 * t328 * t2331 * t3971;
    (t44206, t45096, t46392, t47184, t50887, t50912, t50935, t50942, t50943, t50948)
}
