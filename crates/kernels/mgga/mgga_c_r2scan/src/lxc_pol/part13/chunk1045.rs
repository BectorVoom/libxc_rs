//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1045/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1045<F: Float>(t3416: F, t6767: F, t1096: F, t19327: F, t11153: F, t1338: F, t6755: F, t19309: F, t3348: F, t792: F, t11002: F, t113: F, t3268: F, t97: F) -> (F, F, F, F, F, F, F) {
    let t37204 = t6767 * t3416;
    let t37209 = t19327 * t1096;
    let t37218 = t1338 * t11153;
    let t37223 = t6755 * t3416;
    let t37226 = t19309 * t1096;
    let t37256 = t3348 * t792;
    let t37257 = t11002 * t37256;
    let t37271 = t97 * t3268 * t113;
    (t37204, t37209, t37218, t37223, t37226, t37257, t37271)
}
