//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 623/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk623(t360: f64, t23: f64, t7241: f64, t174: f64, t358: f64, t1556: f64, t357: f64, t1589: f64, t375: f64, t89: f64, t1636: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7741 = t360 * t360;
    let t7742 = 1.0_f64 / t7741;
    let t7750 = t23 * t7241;
    let t7760 = 1.0_f64 / t174 / t358;
    let t7763 = 1.0_f64 / t1556 / t357;
    let t7771 = t89 * t375 * t1589;
    let t7773 = t1636 * t355;
    (t7741, t7742, t7750, t7760, t7763, t7771, t7773)
}
