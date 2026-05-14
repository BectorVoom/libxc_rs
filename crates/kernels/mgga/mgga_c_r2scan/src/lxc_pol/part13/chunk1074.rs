//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1074/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1074<F: Float>(t40460: F, t3579: F, t38289: F, t1563: F, t2867: F, t10997: F, t3275: F, t11506: F, t37313: F, t38365: F, t3348: F, t983: F, t11002: F, t3269: F, t37473: F, t37477: F, t37481: F, t40443: F, t40444: F, t40446: F, t40448: F, t40451: F, t40457: F) -> (F, F, F, F, F, F) {
    let t40461 = 0.43368970657079495312e-4 * t40460;
    let t40463 = t3579 * t38289 / 4.0;
    let t40464 = t2867 * t1563;
    let t40467 = 45.0 / 64.0 * t3275 * t10997 * t40464;
    let t40469 = 3.0 / 2.0 * t11506 * t37313;
    let t40471 = t3579 * t38365 / 2.0;
    let t40472 = t3348 * t983;
    let t40473 = t11002 * t40472;
    let t40475 = 5.0 / 8.0 * t3269 * t40473;
    let t40476 = -t37473 - 0.70441376091769752086e-2 * t37477 - t40443 - t40444 + t40446 + t40448 - 0.15243824895787514157e-3 * t40451 - t40457 + t40461 + t40463 + t40467 - t40469 - t40471 + t37481 + t40475;
    (t40463, t40467, t40469, t40471, t40475, t40476)
}
