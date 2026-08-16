//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2696/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2696(t5778: f64, t9593: f64, t39750: f64, t39756: f64, t39760: f64, t4144: f64, t46992: f64, t46996: f64, t46998: f64, t47003: f64, t48252: f64, t48254: f64, t48256: f64, t5541: f64) -> f64 {
    let t49575 = t5778 * t9593;
    let t49579 = 6.0_f64 * t4144 * t49575 * t5541 + t39750 + t39756 + t39760 + t46992 + t46996 - t46998 + t47003 - t48252 + t48254 - t48256;
    t49579
}
