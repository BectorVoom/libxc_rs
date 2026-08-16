//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2772/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2772(t5572: f64, t9541: f64, t4233: f64, t776: f64, t5527: f64, t828: f64, t5611: f64, t5624: f64, t9601: f64, t1512: f64, t47092: f64, t119: f64, t13222: f64, t13228: f64, t210: f64, t2571: f64, t2643: f64, t2647: f64, t41009: f64, t41053: f64, t4178: f64, t46587: f64, t46595: f64, t46611: f64, t46616: f64, t46618: f64, t46644: f64, t46649: f64, t46658: f64, t47039: f64, t58090: f64) -> (f64, f64) {
    let t58550 = t9541 * t5572;
    let t58552 = t776 * t4233;
    let t58557 = t5527 * t828;
    let t58569 = t5611 * t828;
    let t58574 = t9601 * t5624;
    let t58576 = t47092 * t1512;
    let t58581 = 7.0_f64 / 576.0_f64 * t46587 + 35.0_f64 / 72.0_f64 * t41009 - 7.0_f64 / 576.0_f64 * t46595 + t2571 * t210 * t119 * t58090 / 8.0_f64 - 35.0_f64 / 216.0_f64 * t58550 - t4178 * t13222 * t13228 * t58552 / 96.0_f64 + 5.0_f64 / 64.0_f64 * t2643 * t47039 * t58557 * t2647 + 7.0_f64 / 144.0_f64 * t46611 - 7.0_f64 / 288.0_f64 * t46616 - 7.0_f64 / 576.0_f64 * t46618 - t4178 * t13222 * t13228 * t46644 / 192.0_f64 + t2643 * t13222 * t58569 * t2647 / 384.0_f64 + 595.0_f64 / 3456.0_f64 * t58574 - 119.0_f64 / 6912.0_f64 * t58576 + 119.0_f64 / 864.0_f64 * t46649 - 119.0_f64 / 1728.0_f64 * t41053 - 7.0_f64 / 288.0_f64 * t46658;
    (t58552, t58581)
}
