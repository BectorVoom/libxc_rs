//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2176/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2176(t1361: f64, t22690: f64, t6330: f64, t80840: f64, t22792: f64, t6347: f64, t80900: f64, t80915: f64, t91387: f64, t93757: f64, t97394: f64, t97398: f64, t97400: f64, t97402: f64, t97404: f64, t97407: f64, t97410: f64, t97412: f64, t97414: f64, t97416: f64, t97419: f64, t97423: f64) -> f64 {
    let t97427 = t80840 * t22690 * t1361 * t6330;
    let t97431 = t22792 * t22690 * t1361 * t6347;
    let t97433 = -t80900 + 7.0_f64 / 144.0_f64 * t97394 - 119.0_f64 / 6912.0_f64 * t80915 - 0.20186378047070195427e-3_f64 * t97398 - 0.28260929265898273598e-2_f64 * t97400 - t91387 - t93757 - 7.0_f64 / 48.0_f64 * t97402 - 0.59347951458386374554e-1_f64 * t97404 - 0.16956557559538964158e-1_f64 * t97407 + 0.24223653656484234512e-2_f64 * t97410 - t97412 / 192.0_f64 + t97414 / 384.0_f64 - 5.0_f64 / 384.0_f64 * t97416 + t97419 / 16.0_f64 - 0.12111826828242117256e-2_f64 * t97423 - 0.14130464632949136799e-2_f64 * t97427 + 0.20186378047070195427e-3_f64 * t97431;
    t97433
}
