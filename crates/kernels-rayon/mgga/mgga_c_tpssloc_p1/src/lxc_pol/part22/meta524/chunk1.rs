//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1993/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1993(t21713: f64, t22424: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t19451: f64, t20293: f64, t20296: f64, t20350: f64, t20698: f64, t20702: f64, t20717: f64, t20720: f64, t4028: f64, t510: f64, t513: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6295: f64, t6468: f64, t652: f64, t7458: f64) -> (f64, f64) {
    let t22425 = t21713 + t22424;
    let t22430 = -t113 * t22425 - 3.0_f64 * t1442 * t6287 - 6.0_f64 * t1459 * t19451 - 3.0_f64 * t1774 * t5450 - 6.0_f64 * t1774 * t5457 + 3.0_f64 * t1778 * t6468 + 3.0_f64 * t1849 * t6295 - t20293 * t510 - 6.0_f64 * t20296 * t510 + t20350 * t574 + t20698 * t513 - 6.0_f64 * t20702 * t652 - 6.0_f64 * t20717 * t652 - 2.0_f64 * t20720 * t652 - 12.0_f64 * t4028 * t5460 - 6.0_f64 * t4028 * t5494 - 6.0_f64 * t5494 * t7458;
    (t22425, t22430)
}
