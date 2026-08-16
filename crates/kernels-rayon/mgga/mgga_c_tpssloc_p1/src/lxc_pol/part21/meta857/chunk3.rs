//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3111/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3111(t64309: f64, t64325: f64, t64342: f64, t64358: f64, t64374: f64, t64389: f64, t64406: f64, t64422: f64, t1117: f64, t51460: f64, t51638: f64, t3313: f64, t3315: f64, t63287: f64) -> (f64, f64, f64) {
    let t64425 = t64309 + t64325 + t64342 + t64358 + t64374 + t64389 + t64406 + t64422;
    let t64433 = 0.2069040516770936012e4_f64 * t51638 * t51460 * t1117;
    let t64436 = 0.32163958997385070134e2_f64 * t3313 * t63287 * t3315;
    (t64425, t64433, t64436)
}
