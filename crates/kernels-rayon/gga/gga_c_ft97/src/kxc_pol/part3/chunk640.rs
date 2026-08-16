//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 640/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk640(t1556: f64, t357: f64, t1636: f64, t355: f64, t364: f64, t89: f64, t1554: f64, t375: f64, t1642: f64, t369: f64, t21: f64, t1586: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7763 = 1.0_f64 / t1556 / t357;
    let t7773 = t1636 * t355;
    let t7775 = t89 * t7773 * t364;
    let t7780 = t375 * t1554;
    let t7793 = t1642 * t369;
    let t7800 = 1.0_f64 / t1556 / t21;
    let t7824 = t378 * t1586;
    (t7763, t7773, t7775, t7780, t7793, t7800, t7824)
}
