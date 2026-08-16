//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 730/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk730(t11472: f64, t11473: f64, t10965: f64, t83: f64, t1825: f64, t3214: f64, t452: f64, t11430: f64, t11432: f64, t11436: f64, t11439: f64, t11444: f64, t11448: f64, t11451: f64, t11455: f64, t11459: f64, t11463: f64, t11467: f64, t11469: f64, t1901: f64, t446: f64) -> f64 {
    let t11474 = t11472 * t11473;
    let t11477 = t83 * t10965;
    let t11481 = t452 * t1825 * t3214;
    let t11484 = t11430 - 2.0_f64 / 9.0_f64 * t1901 * t11432 - t11436 - 2.0_f64 / 3.0_f64 * t1901 * t11439 + t1901 * t11444 / 9.0_f64 - t11448 + t1901 * t11451 / 9.0_f64 - t446 * t11455 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t11459 - t446 * t11463 / 3.0_f64 + t11467 - 4.0_f64 / 9.0_f64 * t1901 * t11469 - 4.0_f64 / 9.0_f64 * t1901 * t11474 + 2.0_f64 / 3.0_f64 * t446 * t11477 + 2.0_f64 / 3.0_f64 * t446 * t11481;
    t11484
}
