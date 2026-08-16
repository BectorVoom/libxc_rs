//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1063/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1063(t2030: f64, t4586: f64, t7815: f64, t30640: f64, t30645: f64, t30647: f64, t30649: f64, t30653: f64, t30658: f64, t34566: f64, t34571: f64, t34575: f64, t34578: f64, t34582: f64, t34586: f64, t34590: f64, t34593: f64, t34595: f64, t34598: f64, t34601: f64) -> f64 {
    let t34604 = t2030 * t7815 * t4586;
    let t34606 = -t34566 - 0.10718504529517434243e-3_f64 * t30640 - 0.17149607247227894789e-2_f64 * t30645 + t34571 + 0.12862205435420921092e-2_f64 * t30647 + 0.64311027177104605458e-3_f64 * t30649 - 0.47172138434406228102e-3_f64 * t30653 - t34575 - t30658 + 0.31448092289604152068e-2_f64 * t34578 - 0.12579236915841660827e-2_f64 * t34582 + 0.18868855373762491241e-2_f64 * t34586 - 0.85748036236139473944e-3_f64 * t34590 - t34593 + t34595 / 16.0_f64 + t34598 / 32.0_f64 + t34601 / 64.0_f64 + t34604 / 128.0_f64;
    t34606
}
