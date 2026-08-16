//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 867/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk867(t1336: f64, t1814: f64, t1838: f64, t1840: f64, t5234: f64, t544: f64, t564: f64, t6378: f64, t6448: f64, t6451: f64, t6454: f64, t6456: f64, t6458: f64) -> f64 {
    let t6460 = 2.0_f64 * t1336 * t6448 - 2.0_f64 * t1336 * t6451 - t1336 * t6454 - t1336 * t6456 + 2.0_f64 * t1814 * t1840 - 2.0_f64 * t1838 * t5234 + t544 * t6458 + t564 * t6378;
    t6460
}
