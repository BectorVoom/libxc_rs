//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1001/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1001(t475: f64, t49921: f64, t10525: f64, t10526: f64, t188: f64, t189: f64, t193: f64, t2487: f64, t46457: f64, t46461: f64, t46463: f64, t46471: f64, t46473: f64, t46480: f64, t46490: f64, t46491: f64, t46497: f64, t46498: f64, t46501: f64, t46504: f64, t46507: f64, t46516: f64, t46521: f64, t46526: f64, t46529: f64, t49841: f64, t6711: f64) -> (f64, f64) {
    let t50668 = t49921 * t475;
    let t50675 = -0.57514388930881124514e0_f64 * t46457 + t46461 - t46463 + t46471 + t46473 + 0.12780975317973583225e0_f64 * t46480 + 0.35750489951850426669e0_f64 * t188 * t189 * t49841 * t193 - t46490 + t46491 + t46497 + t46498 + t46501 - t46504 - t46507 + 0.87421871174939309263e2_f64 * t2487 * t6711 * t50668 + t46516 - t46521 - t46526 - 0.42900587942220512004e1_f64 * t10525 * t10526 * t50668 - t46529;
    (t50668, t50675)
}
