//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 701/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk701(t11050: f64, t447: f64, t446: f64, t1588: f64, t925: f64, t7824: f64, t7800: f64, t920: f64, t1559: f64) -> (f64, f64, f64, f64) {
    let t11051 = t447 * t11050;
    let t11052 = t446 * t11051;
    let t11054 = t925 * t1588;
    let t11055 = t7824 * t11054;
    let t11056 = t446 * t11055;
    let t11058 = t7800 * t920;
    let t11059 = t11058 * t1559;
    (t11052, t11054, t11056, t11059)
}
