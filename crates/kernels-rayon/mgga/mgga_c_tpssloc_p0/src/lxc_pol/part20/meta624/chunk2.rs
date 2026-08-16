//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2248/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2248(t4119: f64, t828: f64, t46528: f64, t842: f64, t4261: f64, t9601: f64, t1516: f64, t40965: f64, t13347: f64, t2697: f64, t119: f64, t13248: f64, t13254: f64, t13350: f64, t13365: f64, t210: f64, t2623: f64, t2643: f64, t2647: f64, t2703: f64, t40992: f64, t41009: f64, t41012: f64, t4172: f64, t46426: f64, t787: f64, t849: f64, t9609: f64, t9990: f64) -> f64 {
    let t46565 = t4119 * t828;
    let t46570 = t46528 * t842;
    let t46573 = t9601 * t4261;
    let t46574 = 119.0_f64 / 1152.0_f64 * t46573;
    let t46577 = t40965 * t1516;
    let t46587 = t2697 * t13347;
    let t46593 = -t787 * t210 * t119 * t46426 / 48.0_f64 - 5.0_f64 / 128.0_f64 * t2643 * t13350 * t46565 * t2647 - t46570 * t849 / 256.0_f64 - t46574 - 5.0_f64 / 128.0_f64 * t4172 * t9609 + 595.0_f64 / 2592.0_f64 * t46577 + 5.0_f64 / 256.0_f64 * t13365 * t2703 - t40992 * t1516 / 768.0_f64 - t9990 * t4261 / 256.0_f64 - t2623 * t13347 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t46587 + 35.0_f64 / 24.0_f64 * t41009 + 7.0_f64 / 12.0_f64 * t41012 + t13254 * t13248 / 512.0_f64;
    t46593
}
