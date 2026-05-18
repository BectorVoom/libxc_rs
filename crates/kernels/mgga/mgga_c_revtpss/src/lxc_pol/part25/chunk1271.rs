//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1271/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1271<F: Float>(t11988: F, t7132: F, t3196: F, t7131: F, t11648: F, t7122: F, t11997: F, t25503: F, t3141: F, t25512: F, t3173: F, t1068: F, t11684: F, t11877: F, t11913: F, t12017: F, t25517: F, t25569: F, t25577: F, t25580: F, t3101: F, t3120: F, t3157: F, t3177: F, t3184: F, t93541: F, t93543: F, t93548: F) -> F {
    let t93555 = t7132 * t11988;
    let t93561 = t3196 * t7131;
    let t93564 = t7122 * t11648;
    let t93567 = t3141 * t25503 * t11997;
    let t93570 = t25512 * t3173;
    let t93572 = -F::new(0.17149607247227894789e-2) * t25517 * t11684 + F::new(0.11433071498151929859e-2) * t93541 - F::new(0.25724410870841842183e-2) * t93543 * t3120 - F::new(0.12862205435420921092e-2) * t25580 * t12017 + F::new(0.12862205435420921092e-2) * t93548 * t11877 - F::new(0.45732285992607719436e-2) * t25577 * t3177 - F::new(0.7622047665434619906e-2) * t25577 * t3184 - F::new(0.19055119163586549765e-3) * t93555 - F::new(0.17149607247227894789e-2) * t25569 * t3101 - F::new(0.28582678745379824648e-2) * t7132 * t11913 + F::new(0.85748036236139473944e-3) * t93561 * t1068 + F::new(0.85748036236139473944e-3) * t93564 - F::new(0.13719685797782315831e-1) * t93567 * t3157 + F::new(0.17149607247227894789e-2) * t93570;
    t93572
}
