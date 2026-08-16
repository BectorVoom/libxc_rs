//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1284/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1284<F: Float>(t25516: F, t3278: F, t11156: F, t11165: F, t11252: F, t11634: F, t11639: F, t11667: F, t11767: F, t11829: F, t25522: F, t25539: F, t27498: F, t27526: F, t27527: F, t27531: F, t3097: F, t3164: F, t3238: F, t7111: F, t93789: F, t93793: F, t93796: F, t93799: F, t93801: F, t93813: F) -> F {
    let t93821 = t3278 * t25516;
    let t93824 = F::cast_from(0.25724410870841842183e-2_f64) * t93789 * t11634 - F::cast_from(0.25724410870841842183e-2_f64) * t93793 * t11252 - F::cast_from(0.12862205435420921092e-2_f64) * t93796 * t3164 - F::cast_from(0.91464571985215438873e-2_f64) * t93799 - F::cast_from(0.28582678745379824648e-3_f64) * t93801 + F::cast_from(0.17149607247227894789e-2_f64) * t25522 * t11639 - t27526 * t27527 * t11165 / F::cast_from(48.0_f64) + t27526 * t27531 * t11156 / F::cast_from(72.0_f64) + t25539 * t3238 / F::cast_from(18.0_f64) - t93813 / F::cast_from(144.0_f64) - t7111 * t11829 / F::cast_from(36.0_f64) + t7111 * t11767 / F::cast_from(48.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t27498 * t11667 + F::cast_from(0.17149607247227894789e-2_f64) * t93821 * t3097;
    t93824
}
