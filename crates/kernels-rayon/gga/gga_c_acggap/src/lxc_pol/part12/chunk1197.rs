//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1197/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1197(t35484: f64, t35486: f64, t35496: f64, t35502: f64, t35513: f64, t35515: f64, t31251: f64, t31254: f64, t31256: f64, t31259: f64, t31262: f64, t31277: f64, t31279: f64, t35490: f64, t35494: f64, t35511: f64, t35519: f64, t35523: f64) -> f64 {
    let t37569 = 0.28582678745379824648e-3_f64 * t35484;
    let t37570 = 0.25724410870841842184e-2_f64 * t35486;
    let t37573 = 0.18868855373762491241e-1_f64 * t35496;
    let t37576 = 0.14291339372689912324e-2_f64 * t35502;
    let t37583 = 0.18868855373762491241e-2_f64 * t35513;
    let t37584 = 0.12862205435420921092e-1_f64 * t35515;
    let t37587 = -t37569 - t37570 - 0.21437009059034868486e-3_f64 * t35490 + 0.42874018118069736972e-3_f64 * t35494 + t37573 + 0.62896184579208304137e-3_f64 * t31251 - 0.17149607247227894789e-2_f64 * t31254 - t37576 - 0.85748036236139473944e-3_f64 * t31256 + 0.78443749999999999999e0_f64 * t31259 + 0.52295833333333333333e0_f64 * t31262 - 0.794625e0_f64 * t31277 - 0.52975e0_f64 * t31279 + 0.18868855373762491241e-2_f64 * t35511 + t37583 + t37584 + 0.64311027177104605458e-2_f64 * t35519 - 0.42874018118069736972e-3_f64 * t35523;
    t37587
}
