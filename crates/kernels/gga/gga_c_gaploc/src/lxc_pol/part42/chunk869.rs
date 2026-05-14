//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 869/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk869<F: Float>(t475: F, t49921: F, t10525: F, t10526: F, t188: F, t189: F, t193: F, t2487: F, t46457: F, t46461: F, t46463: F, t46471: F, t46473: F, t46480: F, t46490: F, t46491: F, t46497: F, t46498: F, t46501: F, t46504: F, t46507: F, t46516: F, t46521: F, t46526: F, t46529: F, t49841: F, t6711: F) -> (F, F) {
    let t50668 = t49921 * t475;
    let t50675 = -0.57514388930881124514e0 * t46457 + t46461 - t46463 + t46471 + t46473 + 0.12780975317973583225e0 * t46480 + 0.35750489951850426669e0 * t188 * t189 * t49841 * t193 - t46490 + t46491 + t46497 + t46498 + t46501 - t46504 - t46507 + 0.87421871174939309263e2 * t2487 * t6711 * t50668 + t46516 - t46521 - t46526 - 0.42900587942220512004e1 * t10525 * t10526 * t50668 - t46529;
    (t50668, t50675)
}
