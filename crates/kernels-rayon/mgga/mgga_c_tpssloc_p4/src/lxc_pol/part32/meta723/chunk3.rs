//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2309/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2309(t24826: f64, t29716: f64, t103218: f64, t103615: f64, t103707: f64, t1216: f64, t24745: f64, t27406: f64, t27453: f64, t27460: f64, t27481: f64, t27484: f64, t27498: f64, t3610: f64, t3612: f64, t7283: f64, t7368: f64, t85918: f64, t85941: f64, t85952: f64, t85963: f64, t94858: f64, t94874: f64, t95069: f64) -> f64 {
    let t103810 = t24826 * t29716;
    let t103829 = -0.18277045187202515961e-2_f64 * t85918 + 0.82246703342411321825e-2_f64 * t85963 * t94874 * t103615 * t1216 - 0.54831135561607547883e-2_f64 * t103810 - t95069 - 0.16449340668482264365e-1_f64 * t7283 * t27453 * t24745 * t27460 - 0.18277045187202515961e-2_f64 * t85941 - 0.80418998823691070228e-1_f64 * t103218 * t7368 + 0.43864908449286038306e-1_f64 * t27406 * t27481 + 0.43864908449286038306e-1_f64 * t27406 * t27484 + 0.6092348395734171987e-3_f64 * t85952 + 2.0_f64 * t3610 * t103707 * t3612 + 0.43864908449286038306e-1_f64 * t94858 * t27498;
    t103829
}
