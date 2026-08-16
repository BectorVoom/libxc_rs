//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1211/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1211<F: Float>(t3222: F, t37632: F, t3327: F, t810: F, t1198: F, t21885: F, t804: F, t4058: F, t6854: F, t332: F, t4395: F, t2157: F, t938: F) -> (F, F, F, F, F, F, F) {
    let t46392 = t37632 * t3222;
    let t47184 = t3327 * t810;
    let t50818 = t1198 * t21885;
    let t50832 = t804 * t1198;
    let t50839 = t4058 * t6854;
    let t50887 = t4395 * t332;
    let t50912 = t2157 * t938;
    (t46392, t47184, t50818, t50832, t50839, t50887, t50912)
}
