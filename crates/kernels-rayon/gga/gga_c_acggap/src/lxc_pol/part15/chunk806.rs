//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 806/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk806(t8829: f64, t7649: f64, t7651: f64, t7653: f64, t7655: f64, t8232: f64, t8801: f64, t8804: f64, t8808: f64, t8811: f64, t8814: f64, t8818: f64, t8821: f64, t8824: f64, t8827: f64) -> f64 {
    let t9309 = 0.84046875e-1_f64 * t8829;
    let t9310 = t7649 + t8801 / 64.0_f64 + t8804 / 96.0_f64 + t8808 / 8.0_f64 + t8811 / 24.0_f64 + 0.22921875e-1_f64 * t8814 + 0.22921875e-1_f64 * t8818 + 0.1528125e-1_f64 * t8821 + 0.22921875e-1_f64 * t8824 + 0.1528125e-1_f64 * t8827 - t9309 + t7651 - t7653 + t7655 + t8232;
    t9310
}
