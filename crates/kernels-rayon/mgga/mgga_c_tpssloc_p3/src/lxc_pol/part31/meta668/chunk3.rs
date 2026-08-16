//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1968/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1968(t81789: f64, t87237: f64, t87243: f64, t87268: f64, t92590: f64, t92599: f64, t92603: f64, t92607: f64, t92614: f64, t92615: f64, t98690: f64, t98694: f64, t98696: f64, t98701: f64, t98703: f64, t98707: f64, t98709: f64, t98711: f64) -> f64 {
    let t101425 = -7.0_f64 / 1152.0_f64 * t98690 - t92590 - t87237 - 119.0_f64 / 1728.0_f64 * t87243 + t92599 + t92603 + t92607 - 0.63250651214153279004e-2_f64 * t81789 + 7.0_f64 / 72.0_f64 * t98694 + 0.16956557559538964158e-1_f64 * t98696 - t87268 - t92614 + t92615 + 0.80745512188280781706e-3_f64 * t98701 - t98703 / 24.0_f64 - 0.24223653656484234512e-2_f64 * t98707 - 7.0_f64 / 24.0_f64 * t98709 - 0.11869590291677274911e0_f64 * t98711;
    t101425
}
