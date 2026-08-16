//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2247/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2247(t1496: f64, t41083: f64, t4257: f64, t9601: f64, t13193: f64, t2697: f64, t13204: f64, t2563: f64, t2379: f64, t40959: f64, t40962: f64, t40966: f64, t40982: f64, t40984: f64, t40988: f64, t40990: f64, t40998: f64, t4119: f64, t820: f64, t843: f64, t9607: f64) -> f64 {
    let t46546 = t41083 * t1496;
    let t46549 = t9601 * t4257;
    let t46550 = 595.0_f64 / 1152.0_f64 * t46549;
    let t46551 = t2697 * t13193;
    let t46558 = t2563 * t13204;
    let t46560 = -35.0_f64 / 384.0_f64 * t40959 + 7.0_f64 / 384.0_f64 * t40962 + 595.0_f64 / 864.0_f64 * t40966 - 119.0_f64 / 1152.0_f64 * t40982 + 7.0_f64 / 1152.0_f64 * t40984 + 35.0_f64 / 192.0_f64 * t40988 + 595.0_f64 / 1152.0_f64 * t40990 + 455.0_f64 / 648.0_f64 * t46546 - 7.0_f64 / 16.0_f64 * t40998 + t46550 - 35.0_f64 / 192.0_f64 * t46551 - 15.0_f64 / 128.0_f64 * t843 * t9607 * t820 * t4119 * t2379 + 7.0_f64 / 48.0_f64 * t46558;
    t46560
}
