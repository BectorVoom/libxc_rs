//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1841/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1841(t1450: f64, t1907: f64, t198: f64, t22483: f64, t22809: f64, t22813: f64, t30122: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t4139: f64, t46980: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t47000: f64, t47003: f64, t5532: f64, t91963: f64) -> f64 {
    let t92465 = 24.0_f64 * t1450 * t1907 * t198 * t22813 - 36.0_f64 * t22483 * t30122 * t4139 + 12.0_f64 * t22809 * t4139 * t5532 + t39747 + t39750 + t39756 + t39760 + t46980 + t46988 + t46992 + t46996 - t46998 - t47000 + t47003 + t91963;
    t92465
}
