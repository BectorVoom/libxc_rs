//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 297/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk297(t1268: f64, t898: f64, t900: f64, t1263: f64, t631: f64, t892: f64, t332: f64, t113: f64, t409: f64, t6: f64, t64: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1270 = t898 * t900 * t1268;
    let t1273 = t892 + t631 * t1263 / 6.0_f64 + t631 * t1270 / 2.0_f64;
    let t1274 = t1273 * t332;
    let t1275 = t1274 * t113;
    let t1299 = t409 * t6;
    let t1300 = t64 * t1299;
    let t1354 = t550 * t6;
    (t1270, t1273, t1274, t1275, t1300, t1354)
}
