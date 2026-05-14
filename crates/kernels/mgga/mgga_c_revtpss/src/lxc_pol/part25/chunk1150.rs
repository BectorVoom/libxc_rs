//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1150/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1150<F: Float>(t11240: F, t11244: F, t11627: F, t25503: F, t11273: F, t25508: F, t25526: F, t3173: F, t11263: F, t7122: F, t11762: F, t7111: F, t25516: F, t3278: F, t11156: F, t11165: F, t11252: F, t11634: F, t11639: F, t11667: F, t11767: F, t11829: F, t25522: F, t25539: F, t27498: F, t27526: F, t27527: F, t27531: F, t3097: F, t3164: F, t3238: F, sigma0: F) -> (F,) {
    let t93789 = t11240 * t11627 * sigma0 * t11244;
    let t93793 = t11240 * t25503 * t11244;
    let t93796 = t11273 * t25508;
    let t93799 = t25526 * t3173;
    let t93801 = t7122 * t11263;
    let t93813 = t7111 * t11762;
    let t93821 = t3278 * t25516;
    let t93824 = 0.25724410870841842183e-2 * t93789 * t11634 - 0.25724410870841842183e-2 * t93793 * t11252 - 0.12862205435420921092e-2 * t93796 * t3164 - 0.91464571985215438873e-2 * t93799 - 0.28582678745379824648e-3 * t93801 + 0.17149607247227894789e-2 * t25522 * t11639 - t27526 * t27527 * t11165 / 48.0 + t27526 * t27531 * t11156 / 72.0 + t25539 * t3238 / 18.0 - t93813 / 144.0 - t7111 * t11829 / 36.0 + t7111 * t11767 / 48.0 - 0.85748036236139473944e-3 * t27498 * t11667 + 0.17149607247227894789e-2 * t93821 * t3097;
    (t93824,)
}
