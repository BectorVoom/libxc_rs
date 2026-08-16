//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 725/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk725(t11416: f64, t11036: f64, t11041: f64, t11043: f64, t11048: f64, t11052: f64, t11056: f64, t11061: f64, t11066: f64, t11070: f64, t11073: f64, t11076: f64, t11395: f64, t11399: f64, t11404: f64, t11408: f64, t11413: f64, t7771: f64, t8190: f64, t8195: f64) -> f64 {
    let t11417 = 2.0_f64 / 9.0_f64 * t11416;
    let t11418 = t8195 / 18.0_f64 - t11036 / 27.0_f64 - t11041 - 2.0_f64 / 81.0_f64 * t11043 - t11048 / 9.0_f64 - t11052 / 3.0_f64 - t11056 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t11061 - t7771 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t11066 + t11070 - t11073 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t11076 - t8190 - t11395 / 6.0_f64 - 2.0_f64 / 9.0_f64 * t11399 + 11.0_f64 / 27.0_f64 * t11404 + t11408 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t11413 - t11417;
    t11418
}
