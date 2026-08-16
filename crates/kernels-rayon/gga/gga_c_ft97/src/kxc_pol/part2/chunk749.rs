//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 749/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk749(t11715: f64, t11771: f64, t457: f64, t91: f64, t11069: f64, t11041: f64, t11048: f64, t11052: f64, t11056: f64, t11061: f64, t11066: f64, t11073: f64, t11659: f64, t7771: f64) -> (f64, f64) {
    let t11772 = t11715 + t11771;
    let t11774 = t91 * t457 * t11772;
    let t11778 = 2.0_f64 / 9.0_f64 * t11069;
    let t11780 = -2.0_f64 * t11041 - t11659 - 2.0_f64 / 9.0_f64 * t11048 - 2.0_f64 / 3.0_f64 * t11052 - 2.0_f64 / 9.0_f64 * t11056 + 4.0_f64 / 9.0_f64 * t11061 + t11774 / 6.0_f64 - 2.0_f64 / 9.0_f64 * t7771 - 4.0_f64 / 9.0_f64 * t11066 + t11778 - 2.0_f64 / 9.0_f64 * t11073;
    (t11774, t11780)
}
