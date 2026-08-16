//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 177/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk177(t140: f64, t550: f64, t554: f64, t133: f64, t399: f64, t540: f64, t543: f64) -> f64 {
    let t141 = 0.1e-59_f64 < t140;
    let t555 = t550 * t554;
    let t558 = piecewise3(t141, 2.0_f64 * t540 - 0.60409133884038297798e0_f64 * t543 * t399 + 0.60409133884038297798e0_f64 * t140 * t399 - t133 * t555, 0.0_f64);
    t558
}
