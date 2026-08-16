//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 798/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk798(t2: f64, t4495: f64, t1587: f64, t432: f64, t15625: f64, t464: f64, t463: f64, t4436: f64, t7750: f64, t4531: f64, t458: f64, t4527: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16390 = t2 * t4495;
    let t16392 = t1587 * t16390 * t432;
    let t16395 = t464 * t15625;
    let t16396 = t463 * t16395;
    let t16399 = t2 * t4436;
    let t16401 = t7750 * t16399 * t432;
    let t16404 = t458 * t4531;
    let t16406 = t458 * t4527;
    (t16392, t16395, t16396, t16401, t16404, t16406)
}
