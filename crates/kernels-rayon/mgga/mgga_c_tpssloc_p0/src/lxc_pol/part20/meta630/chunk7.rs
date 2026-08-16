//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2291/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2291(t2631: f64, t776: f64, t13297: f64, t9573: f64, t13080: f64, t9638: f64, t13222: f64, t13228: f64, t13262: f64, t13365: f64, t210: f64, t2379: f64, t2643: f64, t2647: f64, t2707: f64, t41427: f64, t41435: f64, t41437: f64, t4158: f64, t4172: f64, t4178: f64, t4180: f64, t4181: f64, t46426: f64, t46693: f64, t47285: f64, t820: f64, t843: f64, t847: f64, t9559: f64, t9976: f64, t9981: f64, t9997: f64) -> f64 {
    let t47320 = t2631 * t776;
    let t47333 = t9573 * t13297;
    let t47353 = t9638 * t13080;
    let t47359 = 7.0_f64 / 4608.0_f64 * t41427 + 3.0_f64 / 128.0_f64 * t13262 * t13222 * t47285 * t47320 - 3.0_f64 / 256.0_f64 * t13262 * t4180 * t4181 * t9976 - 3.0_f64 / 4.0_f64 * t9559 * t210 * t4158 * t2379 - 7.0_f64 / 16.0_f64 * t47333 + t2643 * t13222 * t46693 * t2647 / 256.0_f64 - 7.0_f64 / 768.0_f64 * t41435 + 7.0_f64 / 192.0_f64 * t41437 + 7.0_f64 / 1536.0_f64 * t4178 * t4180 * t4181 * t9981 - t843 * t847 * t820 * t46426 / 768.0_f64 - t13365 * t2707 / 256.0_f64 - t4172 * t9997 / 768.0_f64 + 35.0_f64 / 384.0_f64 * t47353 - 3.0_f64 / 128.0_f64 * t4178 * t13222 * t13228 * t47320;
    t47359
}
