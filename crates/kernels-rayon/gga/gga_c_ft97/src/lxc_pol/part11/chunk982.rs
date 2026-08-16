//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 982/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk982(t1775: f64, t9221: f64, t9214: f64, t2103: f64, t8282: f64, t1554: f64, t1984: f64, t2: f64, t9181: f64, t11176: f64, t151: f64, t11761: f64, t12791: f64, t12823: f64, t2102: f64, t3499: f64, t3506: f64, t37264: f64, t37269: f64, t37311: f64, t39694: f64, t39698: f64, t39713: f64, t39730: f64, t39739: f64, t40323: f64, t462: f64, t9192: f64, t9217: f64) -> (f64, f64) {
    let t40447 = t1775 * t9221;
    let t40449 = t1775 * t9214;
    let t40451 = t8282 * t2103;
    let t40465 = t1554 * t1984;
    let t40466 = t40465 * t2;
    let t40476 = t1775 * t9181;
    let t40485 = 280.0_f64 / 81.0_f64 * t11176 * t151;
    let t40486 = 8.0_f64 / 3.0_f64 * t40447 - 8.0_f64 / 3.0_f64 * t40449 + 16.0_f64 / 9.0_f64 * t40451 + 4.0_f64 / 3.0_f64 * t462 * t2102 * t39739 + 2.0_f64 * t462 * t2102 * t39730 - 8.0_f64 / 9.0_f64 * t462 * t3499 * t37269 - 20.0_f64 / 9.0_f64 * t462 * t12823 * t37311 - 8.0_f64 / 3.0_f64 * t462 * t40466 * t39694 - 16.0_f64 / 3.0_f64 * t462 * t9192 * t39698 + 8.0_f64 / 3.0_f64 * t462 * t3506 * t37264 - 4.0_f64 / 3.0_f64 * t40476 - 4.0_f64 * t462 * t9217 * t39713 - 8.0_f64 * t11761 * t12791 * t40323 + t40485;
    (t40465, t40486)
}
