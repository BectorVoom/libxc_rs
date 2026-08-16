//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2025/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2025(t84536: f64, t91383: f64, t91394: f64, t93753: f64, t97394: f64, t97398: f64, t97400: f64, t97402: f64, t97404: f64, t97407: f64, t97410: f64, t97412: f64, t97414: f64, t97416: f64, t97419: f64, t97423: f64, t97427: f64, t97431: f64) -> f64 {
    let t102732 = -t84536 + 7.0_f64 / 72.0_f64 * t97394 - t91383 - 0.40372756094140390853e-3_f64 * t97398 - 0.56521858531796547194e-2_f64 * t97400 - t93753 - 119.0_f64 / 1728.0_f64 * t91394 - 7.0_f64 / 24.0_f64 * t97402 - 0.11869590291677274911e0_f64 * t97404 - 0.33913115119077928317e-1_f64 * t97407 + 0.48447307312968469024e-2_f64 * t97410 - t97412 / 96.0_f64 + t97414 / 192.0_f64 - 5.0_f64 / 192.0_f64 * t97416 + t97419 / 8.0_f64 - 0.24223653656484234512e-2_f64 * t97423 - 0.28260929265898273597e-2_f64 * t97427 + 0.40372756094140390853e-3_f64 * t97431;
    t102732
}
