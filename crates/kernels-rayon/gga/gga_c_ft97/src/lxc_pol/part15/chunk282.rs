//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 282/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk282(t1273: f64, t332: f64, t113: f64, t6: f64, t695: f64, t224: f64, t817: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1274 = t1273 * t332;
    let t1275 = t1274 * t113;
    let t1416 = t695 * t6;
    let t1417 = t224 * t1416;
    let t1471 = t817 * t6;
    let t1472 = t285 * t1471;
    (t1274, t1275, t1416, t1417, t1471, t1472)
}
