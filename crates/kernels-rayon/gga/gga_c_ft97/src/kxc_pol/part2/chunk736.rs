//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 736/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk736(t3172: f64, t376: f64, t89: f64, t1755: f64, t979: f64, t452: f64, t488: f64, t3052: f64, t447: f64, t499: f64, t1637: f64, t973: f64) -> (f64, f64, f64, f64) {
    let t11567 = 2.0_f64 / 9.0_f64 * t89 * t376 * t3172;
    let t11568 = t979 * t1755;
    let t11570 = t452 * t488 * t11568;
    let t11574 = t447 * t499 * t3052;
    let t11578 = t89 * t1637 * t973;
    (t11567, t11570, t11574, t11578)
}
