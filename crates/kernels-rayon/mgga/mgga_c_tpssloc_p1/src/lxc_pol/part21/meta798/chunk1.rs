//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2773/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2773(t13257: f64, t4166: f64, t4184: f64, t10007: f64, t13222: f64, t13251: f64, t13262: f64, t13263: f64, t13312: f64, t13350: f64, t16891: f64, t16944: f64, t16949: f64, t2633: f64, t2643: f64, t2645: f64, t2647: f64, t41063: f64, t4178: f64, t4180: f64, t46597: f64, t46661: f64, t46663: f64, t46668: f64, t46675: f64, t46677: f64, t46679: f64, t46686: f64, t47017: f64, t5591: f64, t5593: f64, t58495: f64, t829: f64) -> f64 {
    let t58616 = t4166 * t13257 * t4184;
    let t58628 = -7.0_f64 / 576.0_f64 * t46661 - 35.0_f64 / 288.0_f64 * t46663 + 7.0_f64 / 1152.0_f64 * t46668 + 7.0_f64 / 2304.0_f64 * t46675 + 7.0_f64 / 1152.0_f64 * t46677 + 35.0_f64 / 96.0_f64 * t46679 + t2643 * t13222 * t47017 * t5591 / 192.0_f64 - 5.0_f64 / 192.0_f64 * t2643 * t13350 * t16944 * t829 - 5.0_f64 / 384.0_f64 * t2643 * t13350 * t16949 * t829 + 7.0_f64 / 6.0_f64 * t46686 - t13251 * t13312 / 768.0_f64 - t13262 * t4180 * t16891 * t13263 / 512.0_f64 + t2643 * t2645 * t58495 * t2647 / 384.0_f64 + t2643 * t2645 * t16891 * t10007 / 768.0_f64 - 7.0_f64 / 576.0_f64 * t58616 + t41063 * t5593 / 384.0_f64 + t4178 * t4180 * t16891 * t2633 / 512.0_f64 + t2643 * t2645 * t46597 * t5591 / 384.0_f64;
    t58628
}
