//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 710/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk710(t27256: f64, t9144: f64, t144: f64, t26597: f64, t1359: f64, t3565: f64, t574: f64, t605: f64, t1391: f64, t2185: f64, t3450: f64, t3052: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27257 = t9144 * t27256;
    let t27260 = t144 * t26597;
    let t27263 = t1359 * t3565;
    let t27265 = t574 * t605 * t27263;
    let t27269 = t2185 * t1391 * t3450;
    let t27273 = t569 * t1391 * t3052;
    (t27257, t27260, t27263, t27265, t27269, t27273)
}
