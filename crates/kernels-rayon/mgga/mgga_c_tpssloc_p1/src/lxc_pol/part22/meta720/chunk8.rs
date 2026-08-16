//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2342/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2342(t13278: f64, t5614: f64, t20963: f64, t9667: f64, t46881: f64, t5587: f64, t13222: f64, t13251: f64, t13350: f64, t16888: f64, t20947: f64, t20993: f64, t210: f64, t2571: f64, t2643: f64, t2645: f64, t2647: f64, t4240: f64, t46952: f64, t46954: f64, t5591: f64, t58642: f64, t58688: f64, t58759: f64, t58761: f64, t58763: f64, t67620: f64, t776: f64, t829: f64) -> f64 {
    let t67976 = t13278 * t5614;
    let t67978 = t9667 * t20963;
    let t67980 = t46881 * t5587;
    let t67988 = -t58642 * t4240 / 1024.0_f64 - t46952 - t46954 + t2643 * t2645 * t67620 * t2647 / 768.0_f64 + 7.0_f64 / 1536.0_f64 * t58759 + 7.0_f64 / 1536.0_f64 * t58761 - 35.0_f64 / 192.0_f64 * t58763 + t2643 * t13222 * t58688 * t5591 / 256.0_f64 + t2571 * t210 * t20993 * t776 / 16.0_f64 + 7.0_f64 / 1536.0_f64 * t67976 - 7.0_f64 / 768.0_f64 * t67978 - 7.0_f64 / 768.0_f64 * t67980 - 5.0_f64 / 128.0_f64 * t13251 * t16888 - 5.0_f64 / 256.0_f64 * t2643 * t13350 * t20947 * t829;
    t67988
}
