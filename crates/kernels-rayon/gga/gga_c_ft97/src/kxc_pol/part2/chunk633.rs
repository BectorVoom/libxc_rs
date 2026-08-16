//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 633/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk633(t1725: f64, t1732: f64, t10: f64, t3050: f64, t83: f64, t1636: f64, t433: f64, t89: f64, t1756: f64, t375: f64, t1586: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8133 = t1725 * t1732;
    let t8189 = t10 * t3050 * t83;
    let t8190 = 14.0_f64 / 81.0_f64 * t8189;
    let t8192 = t89 * t1636 * t433;
    let t8195 = t89 * t375 * t1756;
    let t8216 = t355 * t1586;
    (t8133, t8189, t8190, t8192, t8195, t8216)
}
