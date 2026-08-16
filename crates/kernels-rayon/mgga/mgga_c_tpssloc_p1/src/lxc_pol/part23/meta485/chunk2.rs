//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1486/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1486(t5: f64, t79711: f64, t112: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t19451: f64, t20347: f64, t20698: f64, t20702: f64, t20717: f64, t22425: f64, t28002: f64, t4028: f64, t510: f64, t5450: f64, t5457: f64, t5494: f64, t6287: f64, t652: f64, t67001: f64, t7458: f64, t77944: f64, t79553: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t79712 = piecewise3(t8, 0.0_f64, t79711);
    let t79713 = t79712 * t112;
    let t79729 = -t113 * (t77944 + t79553) - 8.0_f64 * t652 * t1774 * t20347 - 12.0_f64 * t5457 * t6287 - 4.0_f64 * t1442 * t22425 - 6.0_f64 * t5450 * t6287 - t79713 * t510 - 24.0_f64 * t7458 * t20717 + 4.0_f64 * t1778 * t20698 - 24.0_f64 * t4028 * t20717 - 12.0_f64 * t19451 * t5494 - 8.0_f64 * t67001 * t1459 - 24.0_f64 * t28002 * t5494 - 24.0_f64 * t4028 * t20702;
    (t79713, t79729)
}
