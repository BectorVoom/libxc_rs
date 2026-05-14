//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 998/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk998<F: Float>(t16932: F, t47391: F, t5293: F, t587: F, t12821: F, t23123: F, t5211: F, t41326: F, t12783: F, t2612: F, t47928: F, t48043: F, t48044: F, t48045: F, t48046: F, t48049: F, t48050: F, t48052: F) -> (F, F, F, F, F) {
    let t48056 = 128.0 / 27.0 * t587 * t5293 * t16932 * t47391;
    let t48059 = 64.0 / 15.0 * t5211 * t23123 * t12821;
    let t48060 = 32.0 / 45.0 * t41326;
    let t48062 = 16.0 / 15.0 * t2612 * t12783;
    let t48063 = t47928 - t48043 - t48044 + t48045 - t48046 + t48049 - t48050 - t48052 - t48056 + t48059 + t48060 - t48062;
    (t48056, t48059, t48060, t48062, t48063)
}
