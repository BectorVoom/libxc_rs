//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2075/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2075(t86588: f64, t22470: f64, t4067: f64, t1453: f64, t2332: f64, t81446: f64, t666: f64, t22473: f64, t2358: f64, t12808: f64, t6530: f64, t81438: f64, t81443: f64, t81445: f64, t86583: f64, t86586: f64) -> f64 {
    let t86589 = 4.0_f64 / 3.0_f64 * t86588;
    let t86590 = t22470 * t4067;
    let t86591 = 2.0_f64 / 3.0_f64 * t86590;
    let t86592 = t1453 * t2332;
    let t86593 = t81446 * t86592;
    let t86595 = t4067 * t666;
    let t86596 = t22473 * t86595;
    let t86598 = t1453 * t2358;
    let t86599 = t22473 * t86598;
    let t86601 = t6530 * t12808;
    let t86603 = -t81438 - t86583 - 2.0_f64 / 3.0_f64 * t81443 + t81445 / 3.0_f64 - 11.0_f64 / 9.0_f64 * t86586 - t86589 + t86591 - 3.0_f64 / 4.0_f64 * t86593 + t86596 / 2.0_f64 + t86599 / 4.0_f64 - t86601 / 8.0_f64;
    t86603
}
