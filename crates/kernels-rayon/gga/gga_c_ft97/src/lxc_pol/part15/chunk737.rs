//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 737/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk737(t20809: f64, t20836: f64, t579: f64, t91: f64, t17351: f64, t17352: f64, t20543: f64, t20547: f64, t20554: f64, t20558: f64, t20562: f64, t20566: f64, t20570: f64, t20658: f64, t20663: f64, t20784: f64) -> (f64, f64, f64) {
    let t20837 = t20809 + t20836;
    let t20839 = t91 * t579 * t20837;
    let t20850 = t20784 / 8.0_f64 + t20839 / 6.0_f64 + t17351 - t17352 - t20658 / 3.0_f64 - 2.0_f64 * t20663 - 2.0_f64 / 9.0_f64 * t20554 + t20558 / 3.0_f64 + t20562 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t20566 - 2.0_f64 / 3.0_f64 * t20570 + 2.0_f64 / 3.0_f64 * t20543 + 2.0_f64 / 9.0_f64 * t20547;
    (t20837, t20839, t20850)
}
