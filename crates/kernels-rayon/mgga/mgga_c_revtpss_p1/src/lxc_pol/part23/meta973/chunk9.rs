//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3307/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3307(t22483: f64, t22809: f64, t39773: f64, t4139: f64, t4140: f64, t46996: f64, t46998: f64, t47003: f64, t48256: f64, t48259: f64, t48261: f64, t5541: f64, t5778: f64, t85905: f64, t85906: f64) -> f64 {
    let t86751 = -3.0_f64 * t22483 * t5541 * t5778 + 3.0_f64 * t22809 * t4139 * t4140 + t39773 + t46996 - t46998 + t47003 - t48256 - t48259 + t48261 - t85905 - t85906;
    t86751
}
