//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1284/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1284(t25516: f64, t3278: f64, t11156: f64, t11165: f64, t11252: f64, t11634: f64, t11639: f64, t11667: f64, t11767: f64, t11829: f64, t25522: f64, t25539: f64, t27498: f64, t27526: f64, t27527: f64, t27531: f64, t3097: f64, t3164: f64, t3238: f64, t7111: f64, t93789: f64, t93793: f64, t93796: f64, t93799: f64, t93801: f64, t93813: f64) -> f64 {
    let t93821 = t3278 * t25516;
    let t93824 = 0.25724410870841842183e-2_f64 * t93789 * t11634 - 0.25724410870841842183e-2_f64 * t93793 * t11252 - 0.12862205435420921092e-2_f64 * t93796 * t3164 - 0.91464571985215438873e-2_f64 * t93799 - 0.28582678745379824648e-3_f64 * t93801 + 0.17149607247227894789e-2_f64 * t25522 * t11639 - t27526 * t27527 * t11165 / 48.0_f64 + t27526 * t27531 * t11156 / 72.0_f64 + t25539 * t3238 / 18.0_f64 - t93813 / 144.0_f64 - t7111 * t11829 / 36.0_f64 + t7111 * t11767 / 48.0_f64 - 0.85748036236139473944e-3_f64 * t27498 * t11667 + 0.17149607247227894789e-2_f64 * t93821 * t3097;
    t93824
}
