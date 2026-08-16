//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 827/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk827(t35318: f64, t9942: f64, t1434: f64, t193: f64, t2506: f64, t35323: f64, t1154: f64, t7484: f64, t743: f64, t6109: f64, t33508: f64, t33513: f64, t35312: f64, t35316: f64, t35321: f64, t35326: f64, t35330: f64, t35334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35336 = t9942 * t35318;
    let t35338 = t1434 * t193 * t35336;
    let t35339 = t2506 * t35323;
    let t35341 = t1434 * t193 * t35339;
    let t35343 = t7484 * t1154;
    let t35344 = t743 * t35343;
    let t35346 = t6109 * t193 * t35344;
    let t35348 = t35312 / 2.0_f64 + t33508 + 2.0_f64 / 9.0_f64 * t35316 + 4.0_f64 / 3.0_f64 * t35321 - 2.0_f64 / 3.0_f64 * t35326 - t35330 / 6.0_f64 - t33513 - t35334 / 9.0_f64 - t35338 + 2.0_f64 / 3.0_f64 * t35341 + t35346 / 12.0_f64;
    (t35336, t35338, t35339, t35341, t35343, t35344, t35346, t35348)
}
