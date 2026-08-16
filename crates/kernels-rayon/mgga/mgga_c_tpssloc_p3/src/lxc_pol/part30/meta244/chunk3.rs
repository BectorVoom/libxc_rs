//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1101/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1101(t6323: f64, t6467: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t4028: f64, t510: f64, t513: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6295: f64, t652: f64) -> (f64, f64) {
    let t6468 = t6323 + t6467;
    let t6470 = -t113 * t6287 - 2.0_f64 * t1442 * t1774 - 4.0_f64 * t1459 * t4028 + 2.0_f64 * t1778 * t1849 - t510 * t5450 - 2.0_f64 * t510 * t5457 + t513 * t6468 - 4.0_f64 * t5460 * t652 - 2.0_f64 * t5494 * t652 + t574 * t6295;
    (t6468, t6470)
}
