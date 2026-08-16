//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 658/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk658(t9059: f64, t9071: f64, t9062: f64, t1882: f64, t2198: f64, t2101: f64, t597: f64, t2133: f64, t604: f64, t24: f64, t7368: f64, t603: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9380 = 2.0_f64 / 9.0_f64 * t9059;
    let t9383 = 28.0_f64 / 81.0_f64 * t9071;
    let t9390 = 2.0_f64 / 9.0_f64 * t9062;
    let t9405 = t1882 * t2198;
    let t9419 = t2101 * t597;
    let t9428 = t2133 * t604;
    let t9432 = t24 * t7368;
    let t9437 = t603 * t603;
    (t9380, t9383, t9390, t9405, t9419, t9428, t9432, t9437)
}
