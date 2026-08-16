//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2250/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2250(t10003: f64, t13222: f64, t13228: f64, t13229: f64, t13251: f64, t13300: f64, t13353: f64, t16935: f64, t2633: f64, t2643: f64, t2645: f64, t41025: f64, t41031: f64, t41467: f64, t4178: f64, t4180: f64, t4182: f64, t4248: f64, t46595: f64, t46597: f64, t46606: f64, t46611: f64, t46616: f64, t46618: f64, t46628: f64, t829: f64, t9616: f64, t9642: f64) -> f64 {
    let t46637 = -7.0_f64 / 384.0_f64 * t46595 - t2643 * t4180 * t46597 * t829 / 1024.0_f64 - t4178 * t13222 * t16935 * t13229 / 64.0_f64 - t4178 * t13222 * t13228 * t46606 / 128.0_f64 + 7.0_f64 / 96.0_f64 * t46611 - 5.0_f64 / 128.0_f64 * t9642 * t13353 - 7.0_f64 / 384.0_f64 * t41025 - 7.0_f64 / 192.0_f64 * t46616 - 7.0_f64 / 384.0_f64 * t46618 + 7.0_f64 / 1536.0_f64 * t41031 + t13251 * t10003 / 256.0_f64 + t4178 * t4180 * t46597 * t4182 / 512.0_f64 - 15.0_f64 / 128.0_f64 * t46628 * t41467 * t4248 * t9616 - t4178 * t2645 * t13300 * t2633 / 128.0_f64;
    t46637
}
