//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1047/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1047(t41806: f64, t1882: f64, t9758: f64, t9741: f64, t9754: f64, t9746: f64, t675: f64, t9568: f64, t713: f64, t9572: f64, t446: f64, t41433: f64, t41437: f64, t41439: f64, t41443: f64, t41797: f64, t41801: f64, t41803: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41807 = 56.0_f64 / 81.0_f64 * t41806;
    let t41808 = t1882 * t9758;
    let t41810 = t1882 * t9741;
    let t41812 = t1882 * t9754;
    let t41814 = t1882 * t9746;
    let t41816 = t9568 * t675;
    let t41817 = t9572 * t713;
    let t41819 = t446 * t41816 * t41817;
    let t41821 = -4.0_f64 / 3.0_f64 * t41433 + 4.0_f64 / 9.0_f64 * t41437 + 4.0_f64 / 9.0_f64 * t41439 - 4.0_f64 / 3.0_f64 * t41443 - t41797 / 6.0_f64 - t41801 + 2.0_f64 / 9.0_f64 * t41803 + t41807 - 2.0_f64 / 9.0_f64 * t41808 - 4.0_f64 / 9.0_f64 * t41810 + 4.0_f64 / 27.0_f64 * t41812 - 4.0_f64 / 27.0_f64 * t41814 + 20.0_f64 / 81.0_f64 * t41819;
    (t41808, t41810, t41812, t41814, t41817, t41819, t41821)
}
