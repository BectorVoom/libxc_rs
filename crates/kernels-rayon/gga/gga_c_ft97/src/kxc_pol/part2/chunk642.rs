//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 642/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk642(t100: f64, t8326: f64, t1822: f64, t1882: f64, t1863: f64, t104: f64, t7943: f64, t89: f64, t1786: f64, t488: f64, t1859: f64, t7954: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8518 = t8326 * t100;
    let t8523 = t1882 * t1822;
    let t8526 = t1882 * t1863;
    let t8534 = 28.0_f64 / 81.0_f64 * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    let t8567 = t1882 * t1859;
    let t8577 = t7954 * t82;
    (t8518, t8523, t8526, t8534, t8557, t8567, t8577)
}
