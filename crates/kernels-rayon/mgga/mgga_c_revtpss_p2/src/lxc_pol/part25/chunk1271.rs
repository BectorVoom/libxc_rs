//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1271/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1271(t11988: f64, t7132: f64, t3196: f64, t7131: f64, t11648: f64, t7122: f64, t11997: f64, t25503: f64, t3141: f64, t25512: f64, t3173: f64, t1068: f64, t11684: f64, t11877: f64, t11913: f64, t12017: f64, t25517: f64, t25569: f64, t25577: f64, t25580: f64, t3101: f64, t3120: f64, t3157: f64, t3177: f64, t3184: f64, t93541: f64, t93543: f64, t93548: f64) -> f64 {
    let t93555 = t7132 * t11988;
    let t93561 = t3196 * t7131;
    let t93564 = t7122 * t11648;
    let t93567 = t3141 * t25503 * t11997;
    let t93570 = t25512 * t3173;
    let t93572 = -0.17149607247227894789e-2_f64 * t25517 * t11684 + 0.11433071498151929859e-2_f64 * t93541 - 0.25724410870841842183e-2_f64 * t93543 * t3120 - 0.12862205435420921092e-2_f64 * t25580 * t12017 + 0.12862205435420921092e-2_f64 * t93548 * t11877 - 0.45732285992607719436e-2_f64 * t25577 * t3177 - 0.7622047665434619906e-2_f64 * t25577 * t3184 - 0.19055119163586549765e-3_f64 * t93555 - 0.17149607247227894789e-2_f64 * t25569 * t3101 - 0.28582678745379824648e-2_f64 * t7132 * t11913 + 0.85748036236139473944e-3_f64 * t93561 * t1068 + 0.85748036236139473944e-3_f64 * t93564 - 0.13719685797782315831e-1_f64 * t93567 * t3157 + 0.17149607247227894789e-2_f64 * t93570;
    t93572
}
