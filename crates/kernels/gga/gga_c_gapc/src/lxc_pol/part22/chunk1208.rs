//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1208/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1208<F: Float>(t12050: F, t12045: F, t12063: F, t12041: F, t12296: F, t12053: F, t12043: F, t12047: F, t12061: F, t12046: F, t12059: F, t12039: F, t12042: F, t12151: F, t36091: F, t36092: F, t36093: F, t37313: F, t37314: F, t7: F) -> (F,) {
    let t37317 = 4.0 * t12050;
    let t37318 = 2.0 * t12045;
    let t37319 = 4.0 * t12063;
    let t37320 = 2.0 * t12041;
    let t37322 = 2.0 * t12296;
    let t37323 = 12.0 * t12053;
    let t37324 = 4.0 * t12043;
    let t37325 = 4.0 * t12047;
    let t37326 = 2.0 * t12061;
    let t37327 = 2.0 * t12046;
    let t37328 = 4.0 * t12059;
    let t37329 = 2.0 * t12039;
    let t37330 = 2.0 * t12042;
    let tv4rho2sigma21 = -t36091 - t36092 + t36093 + t7 * (t37313 + t37314) + t37317 - t37318 + t37319 - t37320 + 2.0 * t12151 + t37322 - t37323 + t37324 + t37325 - t37326 - t37327 + t37328 + t37329 - t37330;
    (tv4rho2sigma21,)
}
