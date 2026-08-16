//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1016/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1016(t12257: f64, t13100: f64, t247: f64, t1222: f64, t1247: f64, t1252: f64, t1261: f64, t13008: f64, t13012: f64, t13015: f64, t13018: f64, t13022: f64, t13029: f64, t13033: f64, t13042: f64, t13048: f64, t13052: f64, t13055: f64, t13058: f64, t13062: f64, t13065: f64, t13069: f64, t13076: f64, t13081: f64, t13086: f64, t13090: f64, t13092: f64, t13095: f64, t3591: f64, t3606: f64, t3613: f64, t3708: f64, t5384: f64) -> f64 {
    let t13102 = t247 * t13100 * t12257;
    let t13105 = -t1222 * t13008 / 48.0_f64 + t13012 / 432.0_f64 - t13015 / 288.0_f64 + t13018 / 216.0_f64 + t1222 * t13022 / 36.0_f64 - 7.0_f64 / 648.0_f64 * t1222 * t13029 + 0.12862205435420921092e-2_f64 * t13033 * t3606 + 0.12862205435420921092e-2_f64 * t13042 * t13048 - 0.12862205435420921092e-2_f64 * t13052 * t13055 - 0.64311027177104605458e-3_f64 * t13058 * t3613 + 0.21437009059034868486e-3_f64 * t13062 * t13065 + 0.64311027177104605458e-3_f64 * t13069 * t1252 + 0.64311027177104605458e-3_f64 * t3708 * t3591 + 0.21437009059034868486e-3_f64 * t1247 * t13076 - 0.85748036236139473944e-3_f64 * t5384 * t13081 - 0.28582678745379824648e-3_f64 * t13086 - 0.57165357490759649295e-3_f64 * t13090 - 0.57165357490759649295e-3_f64 * t13092 + 0.12862205435420921092e-2_f64 * t5384 * t13095 - 0.63517063878621832552e-3_f64 * t1261 * t13102;
    t13105
}
