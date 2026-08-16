//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2179/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2179(t80940: f64, t80957: f64, t80971: f64, t91400: f64, t91403: f64, t91404: f64, t93760: f64, t97435: f64, t97437: f64, t97439: f64, t97444: f64, t97447: f64, t97450: f64, t97453: f64, t97456: f64, t97459: f64, t97461: f64, t97463: f64) -> f64 {
    let t97465 = -0.48447307312968469024e-2_f64 * t97435 - t97437 / 48.0_f64 + 0.84782787797694820792e-2_f64 * t97439 - t93760 - 0.13565246047631171327e0_f64 * t91400 + t91403 + 0.16956557559538964159e-1_f64 * t91404 - 0.11304371706359309439e-1_f64 * t80940 + 0.14130464632949136799e-2_f64 * t97444 + 0.16956557559538964158e-1_f64 * t97447 + 0.84782787797694820792e-2_f64 * t97450 - t80957 + t80971 - t97453 / 4.0_f64 + t97456 / 8.0_f64 - 0.67826230238155856634e-1_f64 * t97459 - t97461 / 256.0_f64 + 0.14130464632949136799e-2_f64 * t97463;
    t97465
}
