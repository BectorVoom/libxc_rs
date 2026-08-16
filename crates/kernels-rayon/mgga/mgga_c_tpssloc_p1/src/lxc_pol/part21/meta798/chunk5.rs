//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2777/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2777(t5614: f64, t9674: f64, t16859: f64, t2639: f64, t13360: f64, t4257: f64, t58181: f64, t816: f64, t13222: f64, t13228: f64, t13254: f64, t13351: f64, t13365: f64, t16928: f64, t16935: f64, t2643: f64, t4178: f64, t46565: f64, t46693: f64, t46930: f64, t46936: f64, t46951: f64, t46953: f64, t46960: f64, t46962: f64, t46974: f64, t46980: f64, t46998: f64, t5591: f64, t831: f64) -> f64 {
    let t58759 = t9674 * t5614;
    let t58761 = t2639 * t16859;
    let t58763 = t13360 * t4257;
    let t58765 = t58181 * t816;
    let t58789 = 7.0_f64 / 2304.0_f64 * t46930 + 7.0_f64 / 2304.0_f64 * t46936 - 119.0_f64 / 3456.0_f64 * t46951 - 119.0_f64 / 3456.0_f64 * t46953 + 7.0_f64 / 2304.0_f64 * t58759 + 7.0_f64 / 2304.0_f64 * t58761 - 35.0_f64 / 288.0_f64 * t58763 - t58765 * t831 / 1536.0_f64 + 5.0_f64 / 192.0_f64 * t13365 * t4257 + 7.0_f64 / 2304.0_f64 * t46960 - 35.0_f64 / 576.0_f64 * t46962 - 7.0_f64 / 288.0_f64 * t46974 - 7.0_f64 / 576.0_f64 * t46980 - t4178 * t13222 * t13228 * t46565 / 96.0_f64 - t13254 * t16928 / 96.0_f64 - t4178 * t13222 * t16935 * t13351 / 96.0_f64 - 7.0_f64 / 1152.0_f64 * t46998 + t2643 * t13222 * t46693 * t5591 / 384.0_f64;
    t58789
}
