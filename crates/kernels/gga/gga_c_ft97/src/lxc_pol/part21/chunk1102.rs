//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1102/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1102<F: Float>(t1882: F, t26833: F, t27235: F, t8392: F, t38953: F, t6627: F, t26860: F, t27330: F, t1384: F, t7800: F, t27217: F, t27222: F, t26919: F, t46862: F, t582: F, t6685: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t106314 = 4.0 / 9.0 * t1882 * t26833;
    let t106319 = 4.0 / 81.0 * t8392 * t27235;
    let t106351 = t38953 * t6627;
    let t106361 = 4.0 / 27.0 * t8392 * t26860;
    let t106384 = 4.0 / 9.0 * t8392 * t27330;
    let t106395 = t1384 * t7800;
    let t106413 = 4.0 / 27.0 * t8392 * t27217;
    let t106415 = 4.0 / 81.0 * t8392 * t27222;
    let t106496 = t46862 * t26919;
    let t106551 = t582 * t6685;
    (t106314, t106319, t106351, t106361, t106384, t106395, t106413, t106415, t106496, t106551)
}
