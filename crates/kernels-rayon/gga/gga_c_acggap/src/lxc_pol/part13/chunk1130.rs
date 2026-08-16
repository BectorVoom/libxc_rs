//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1130/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1130(t1988: f64, t8486: f64, t1967: f64, t8838: f64, t4352: f64, t535: f64, t598: f64, t7656: f64, t1089: f64, t12473: f64, t2288: f64, t31251: f64, t31256: f64, t31259: f64, t35485: f64, t35486: f64, t35490: f64, t35494: f64, t35497: f64, t35499: f64, t35503: f64, t35506: f64, t35507: f64, t35508: f64, t35511: f64) -> f64 {
    let t35513 = t1988 * t8486;
    let t35514 = 0.94344276868812456204e-3_f64 * t35513;
    let t35515 = t1967 * t8838;
    let t35519 = t598 * t4352 * t535 * t7656;
    let t35523 = t598 * t1089 * t12473 * t2288;
    let t35525 = -t35485 - 0.12862205435420921092e-2_f64 * t35486 - 0.10718504529517434243e-3_f64 * t35490 + 0.21437009059034868486e-3_f64 * t35494 + t35497 + 0.31448092289604152067e-3_f64 * t31251 - t35499 - t35503 - 0.42874018118069736972e-3_f64 * t31256 + 0.39221875e0_f64 * t31259 + t35506 - t35507 - t35508 + 0.94344276868812456204e-3_f64 * t35511 + t35514 + 0.64311027177104605458e-2_f64 * t35515 + 0.32155513588552302729e-2_f64 * t35519 - 0.21437009059034868486e-3_f64 * t35523;
    t35525
}
