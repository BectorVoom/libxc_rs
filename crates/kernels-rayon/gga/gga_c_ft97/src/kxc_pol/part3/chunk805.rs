//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 805/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk805(t11031: f64, t11043: f64, t11076: f64, t11404: f64, t11778: f64, t11798: f64, t16464: f64, t16469: f64, t16472: f64, t16476: f64, t8455: f64, t16503: f64, t16515: f64, t16523: f64) -> f64 {
    let t16531 = -t11031 - 8.0_f64 / 81.0_f64 * t11043 + t11778 - 8.0_f64 / 27.0_f64 * t11076 - t8455 + t16464 / 6.0_f64 + 4.0_f64 / 27.0_f64 * t11404 - t11798 - t16469 / 12.0_f64 - t16472 / 6.0_f64 + t16476 / 8.0_f64;
    let t16533 = t16503 + t16515 + t16523 + t16531;
    t16533
}
