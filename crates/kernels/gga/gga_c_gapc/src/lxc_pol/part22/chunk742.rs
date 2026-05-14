//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 742/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk742<F: Float>(t9269: F, t9272: F, t1839: F, t6: F, t134: F, t1509: F, t2998: F, t3004: F, t1: F, t1453: F, t519: F, t1030: F, t3008: F, t1403: F, t3005: F, t3007: F, t9079: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9273 = t9269 * t9272;
    let t9275 = t1839 * t6;
    let t9276 = t134 * t1509;
    let t9277 = t9275 * t9276;
    let t9278 = t2998 * t9277;
    let t9279 = t3004 * t9278;
    let t9281 = t1453 * t1;
    let t9282 = t519 * t9281;
    let t9283 = t1030 * t9282;
    let t9284 = t9283 * t3008;
    let t9286 = t134 * t1403;
    let t9287 = t3005 * t9286;
    let t9288 = t2998 * t9287;
    let t9289 = t3004 * t9288;
    let t9291 = t9079 * t3007;
    (t9273, t9278, t9279, t9281, t9282, t9284, t9288, t9289, t9291)
}
