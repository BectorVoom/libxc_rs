//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2254/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2254(t13225: f64, t9638: f64, t13177: f64, t13222: f64, t13231: f64, t13242: f64, t13254: f64, t13262: f64, t13263: f64, t1484: f64, t1495: f64, t210: f64, t2643: f64, t2686: f64, t40971: f64, t41161: f64, t4180: f64, t4181: f64, t46644: f64, t46675: f64, t46677: f64, t46679: f64, t46686: f64, t46692: f64, t46693: f64, t820: f64, t829: f64, t843: f64, t9458: f64, t9642: f64, t9661: f64) -> f64 {
    let t46698 = t9638 * t13225;
    let t46716 = -t13177 * t2686 / 1024.0_f64 + 7.0_f64 / 1536.0_f64 * t46675 + 7.0_f64 / 768.0_f64 * t46677 + 35.0_f64 / 64.0_f64 * t46679 + 35.0_f64 / 128.0_f64 * t843 * t40971 * t820 * t1484 * t9458 + 7.0_f64 / 4.0_f64 * t46686 + 5.0_f64 / 4.0_f64 * t41161 * t210 * t1495 * t9458 - t2643 * t46692 * t46693 * t829 / 1024.0_f64 - 7.0_f64 / 192.0_f64 * t46698 + t2643 * t13222 * t46644 * t829 / 256.0_f64 + t9642 * t13225 / 128.0_f64 - t13254 * t13231 / 64.0_f64 - 3.0_f64 / 512.0_f64 * t13262 * t4180 * t13242 * t13263 - t2643 * t4180 * t4181 * t9661 / 3072.0_f64;
    t46716
}
