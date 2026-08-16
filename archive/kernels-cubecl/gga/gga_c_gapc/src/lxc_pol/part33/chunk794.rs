//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 794/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk794<F: Float>(t3008: F, t9283: F, t134: F, t1403: F, t3005: F, t2998: F, t3004: F, t3007: F, t9079: F, t1404: F, t2982: F, t3084: F) -> (F, F, F, F, F, F, F) {
    let t9284 = t9283 * t3008;
    let t9286 = t134 * t1403;
    let t9287 = t3005 * t9286;
    let t9288 = t2998 * t9287;
    let t9289 = t3004 * t9288;
    let t9291 = t9079 * t3007;
    let t9292 = t3004 * t9291;
    let t9294 = t2982 * t1404;
    let t9295 = t3084 * t9294;
    (t9284, t9288, t9289, t9291, t9292, t9294, t9295)
}
