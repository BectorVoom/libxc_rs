//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 353/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk353(t1773: f64, t1776: f64, t1778: f64, t1783: f64, t1788: f64, t1793: f64, t1797: f64, t1802: f64, t1806: f64, t462: f64, t92: f64, t457: f64, t91: f64) -> (f64, f64) {
    let t1808 = t1773 + 2.0_f64 / 9.0_f64 * t1776 + 2.0_f64 / 3.0_f64 * t1778 - 2.0_f64 / 9.0_f64 * t462 * t1783 + 2.0_f64 / 3.0_f64 * t462 * t1788 + 2.0_f64 / 3.0_f64 * t462 * t1793 - t462 * t1797 / 3.0_f64 + 2.0_f64 * t92 * t1802 - t92 * t1806;
    let t1810 = t91 * t457 * t1808;
    (t1808, t1810)
}
