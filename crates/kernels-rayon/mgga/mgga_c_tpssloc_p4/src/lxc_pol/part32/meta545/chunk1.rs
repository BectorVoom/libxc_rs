//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1896/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1896(t1238: f64, t14980: f64, t1761: f64, t2155: f64, t24589: f64, t24880: f64, t27406: f64, t27422: f64, t27424: f64, t27427: f64, t27434: f64, t27438: f64, t27441: f64, t27446: f64, t27742: f64, t27747: f64, t27752: f64, t27755: f64, t3487: f64, t498: f64, t7283: f64, t7288: f64, t8061: f64) -> f64 {
    let t27757 = t27422 * t498 + t27424 * t498 - 0.27415567780803773942e-2_f64 * t7283 * t27427 + 0.73108180748810063843e-2_f64 * t27406 * t7288 + 0.27415567780803773942e-2_f64 * t24589 * t27434 + 0.27415567780803773942e-2_f64 * t24589 * t27438 + 0.27415567780803773942e-2_f64 * t24589 * t27441 - 0.54831135561607547884e-2_f64 * t24589 * t27446 - t14980 * t2155 - t1238 * t27742 + 2.0_f64 * t3487 * t8061 + 2.0_f64 * t1238 * t27747 - t24880 * t1761 - 0.82246703342411321825e-2_f64 * t7283 * t27752 - 0.27415567780803773942e-2_f64 * t27755;
    t27757
}
