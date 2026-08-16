//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1069/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1069<F: Float>(t12646: F, t247: F, t3719: F, t3367: F, t414: F, t66: F, t12257: F, t1222: F, t1247: F, t1252: F, t1261: F, t13008: F, t13012: F, t13015: F, t13018: F, t13022: F, t13029: F, t13033: F, t13042: F, t13048: F, t13052: F, t13055: F, t13058: F, t13062: F, t13065: F, t13069: F, t13076: F, t13081: F, t13086: F, t13090: F, t13092: F, t3591: F, t3606: F, t3613: F, t3708: F, t5384: F) -> (F, F, F) {
    let t13095 = t247 * t3719 * t12646;
    let t13099 = F::cast_from(1.0_f64) / t414 / t3367;
    let t13100 = t66 * t13099;
    let t13102 = t247 * t13100 * t12257;
    let t13105 = -t1222 * t13008 / F::cast_from(48.0_f64) + t13012 / F::cast_from(432.0_f64) - t13015 / F::cast_from(288.0_f64) + t13018 / F::cast_from(216.0_f64) + t1222 * t13022 / F::cast_from(36.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1222 * t13029 + F::cast_from(0.12862205435420921092e-2_f64) * t13033 * t3606 + F::cast_from(0.12862205435420921092e-2_f64) * t13042 * t13048 - F::cast_from(0.12862205435420921092e-2_f64) * t13052 * t13055 - F::cast_from(0.64311027177104605458e-3_f64) * t13058 * t3613 + F::cast_from(0.21437009059034868486e-3_f64) * t13062 * t13065 + F::cast_from(0.64311027177104605458e-3_f64) * t13069 * t1252 + F::cast_from(0.64311027177104605458e-3_f64) * t3708 * t3591 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t13076 - F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t13081 - F::cast_from(0.28582678745379824648e-3_f64) * t13086 - F::cast_from(0.57165357490759649295e-3_f64) * t13090 - F::cast_from(0.57165357490759649295e-3_f64) * t13092 + F::cast_from(0.12862205435420921092e-2_f64) * t5384 * t13095 - F::cast_from(0.63517063878621832552e-3_f64) * t1261 * t13102;
    (t13095, t13102, t13105)
}
