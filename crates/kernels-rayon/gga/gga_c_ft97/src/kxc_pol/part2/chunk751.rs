//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 751/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk751(t11644: f64, t11656: f64, t11780: f64, t11799: f64, t103: f64, t82: f64, t3205: f64, t8372: f64, t100: f64, t1587: f64, t487: f64, t942: f64) -> (f64, f64, f64, f64, f64) {
    let t11801 = t11644 + t11656 + t11780 + t11799;
    let t11803 = t82 * t11801 * t103;
    let t11807 = t8372 * t3205;
    let t11810 = t1587 * t100;
    let t11811 = t487 * t942;
    (t11801, t11803, t11807, t11810, t11811)
}
