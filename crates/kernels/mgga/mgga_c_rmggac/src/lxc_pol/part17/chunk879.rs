//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 879/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk879<F: Float>(t41407: F, t649: F, t6561: F, t35960: F, t6564: F, t40928: F, t6523: F, t2073: F, t46177: F, t1756: F, t2079: F, t262: F, t265: F, t664: F, t570: F, t8704: F) -> (F, F, F, F, F, F, F) {
    let t46480 = t41407 * t649 * t6561;
    let t46483 = t35960 * t649 * t6564;
    let t46486 = t40928 * t649 * t6523;
    let t46488 = t2073 * t46177;
    let t46492 = t2079 * t262 * t265 * t1756;
    let t46494 = t664 * t1756;
    let t46501 = t8704 * t570;
    (t46480, t46483, t46486, t46488, t46492, t46494, t46501)
}
