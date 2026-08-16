//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 557/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk557(t1073: f64, t358: f64, t2266: f64, t363: f64, t2281: f64, t637: f64, t643: f64, t1640: f64, t2289: f64, t3042: f64, t3045: f64, t3048: f64, t3054: f64, t3359: f64, t3363: f64, t383: f64) -> (f64, f64, f64, f64, f64) {
    let t3635 = t1073 * t358;
    let t3637 = t2266 * t3635 * t363;
    let t3640 = t2281 * t1073;
    let t3642 = t637 * t3640 * t643;
    let t3653 = -0.117377e0_f64 * t3359 * t383 + 0.234754e0_f64 * t3363 + t2289 + 0.4814361111111111111e-1_f64 * t1640 + 0.4814361111111111111e-1_f64 * t3042 - 0.9628722222222222222e-1_f64 * t3045 + 0.28886166666666666666e0_f64 * t3048 - 0.28886166666666666666e0_f64 * t3054;
    (t3635, t3637, t3640, t3642, t3653)
}
