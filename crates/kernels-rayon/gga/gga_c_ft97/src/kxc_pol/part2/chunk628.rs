//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 628/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk628(t30: f64, t7911: f64, t25: f64, t1663: f64, t37: f64, t78: f64, t23: f64, t2999: f64, t26: f64, t1771: f64, t380: f64, t1644: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7913 = 1.0_f64 / t30 / t7911;
    let t7914 = t25 * t7913;
    let t7918 = t37 * t1663;
    let t7919 = t7918 * t78;
    let t7943 = t2999 * t23;
    let t7944 = t26 * t7943;
    let t7945 = 28.0_f64 / 27.0_f64 * t7944;
    let t7946 = t1771 * t380;
    let t7948 = t458 * t1644;
    (t7914, t7919, t7943, t7944, t7945, t7946, t7948)
}
