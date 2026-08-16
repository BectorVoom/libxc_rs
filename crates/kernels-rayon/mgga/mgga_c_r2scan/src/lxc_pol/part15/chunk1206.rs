//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1206/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1206(t11506: f64, t37313: f64, t3579: f64, t38365: f64, t3348: f64, t983: f64, t11002: f64, t3269: f64, t37473: f64, t37477: f64, t37481: f64, t40443: f64, t40444: f64, t40446: f64, t40448: f64, t40451: f64, t40457: f64, t40461: f64, t40463: f64, t40467: f64) -> (f64, f64, f64, f64) {
    let t40469 = 3.0_f64 / 2.0_f64 * t11506 * t37313;
    let t40471 = t3579 * t38365 / 2.0_f64;
    let t40472 = t3348 * t983;
    let t40473 = t11002 * t40472;
    let t40475 = 5.0_f64 / 8.0_f64 * t3269 * t40473;
    let t40476 = -t37473 - 0.70441376091769752086e-2_f64 * t37477 - t40443 - t40444 + t40446 + t40448 - 0.15243824895787514157e-3_f64 * t40451 - t40457 + t40461 + t40463 + t40467 - t40469 - t40471 + t37481 + t40475;
    (t40469, t40471, t40475, t40476)
}
