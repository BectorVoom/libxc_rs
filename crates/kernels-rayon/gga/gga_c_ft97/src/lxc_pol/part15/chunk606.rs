//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 606/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk606(t11298: f64, t419: f64, t122: f64, t409: f64, t371: f64, t11174: f64, t17: f64, t110: f64, t1786: f64, t463: f64, t488: f64, t100: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11299 = t419 * t11298;
    let t11360 = t409 * t122;
    let t11361 = t371 * t11360;
    let t11401 = t11174 * t17;
    let t11468 = t1786 * t110;
    let t11472 = t463 * t488;
    let t11490 = t370 * t100;
    (t11299, t11361, t11401, t11468, t11472, t11490)
}
