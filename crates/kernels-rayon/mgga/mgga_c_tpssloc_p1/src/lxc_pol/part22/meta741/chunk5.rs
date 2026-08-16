//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2448/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2448(t135: f64, t21446: f64, t973: f64, t41863: f64, t48097: f64, t48103: f64, t68452: f64, t68454: f64, t68460: f64, t68464: f64, t68468: f64, t68472: f64, t68500: f64, t68502: f64, t68504: f64, t68506: f64, t68515: f64, t68518: f64, t68523: f64, t68527: f64, t68530: f64, t68536: f64, t68541: f64) -> (f64, f64) {
    let t69579 = t973 * t135 * t21446;
    let t69598 = t48097 + 2.0_f64 / 3.0_f64 * t68452 - t68454 / 9.0_f64 - 40.0_f64 / 27.0_f64 * t48103 - t68460 - t68464 + t68468 / 6.0_f64 + t68472 / 6.0_f64 - 40.0_f64 / 81.0_f64 * t41863 - 4.0_f64 / 81.0_f64 * t68500 - t68502 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t68504 + 2.0_f64 / 9.0_f64 * t68506 + 3.0_f64 * t68515 - t68518 - 8.0_f64 / 9.0_f64 * t68523 + 2.0_f64 / 9.0_f64 * t68527 + 14.0_f64 / 81.0_f64 * t68530 - t68536 / 3.0_f64 + t68541 / 18.0_f64;
    (t69579, t69598)
}
