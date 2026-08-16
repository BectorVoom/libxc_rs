//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3236/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3236(t1486: f64, t1494: f64, t19680: f64, t21695: f64, t21698: f64, t21699: f64, t21702: f64, t22673: f64, t22676: f64, t4181: f64, t4187: f64, t4217: f64, t4238: f64, t5826: f64, t5827: f64, t5854: f64, t641: f64, t85: f64) -> f64 {
    let t85206 = -t22673 * t641 / 12.0_f64 - t19680 * t1486 * t85 / 4.0_f64 - t21698 * t1486 * t85 / 4.0_f64 - t5826 * t4217 * t85 / 4.0_f64 - t22676 * t641 / 4.0_f64 - t21695 * t1494 / 4.0_f64 - t21699 * t1494 / 4.0_f64 - t21702 * t1494 / 4.0_f64 - t5827 * t4238 / 4.0_f64 - t4181 * t5854 * t85 / 4.0_f64 - t4187 * t5854 * t85 / 4.0_f64;
    t85206
}
