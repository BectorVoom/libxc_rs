//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 624/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk624(t364: f64, t7773: f64, t89: f64, t1546: f64, t1581: f64, t1554: f64, t375: f64, t1560: f64, t1642: f64, t369: f64, t1556: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7775 = t89 * t7773 * t364;
    let t7778 = t89 * t1546 * t1581;
    let t7780 = t375 * t1554;
    let t7782 = t89 * t7780 * t1560;
    let t7793 = t1642 * t369;
    let t7800 = 1.0_f64 / t1556 / t21;
    (t7775, t7778, t7780, t7782, t7793, t7800)
}
