//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1074/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1074(t1181: f64, t5265: f64, t604: f64, t8463: f64, t2264: f64, t30792: f64, t4567: f64, t8511: f64, t30715: f64, t34694: f64, t34698: f64, t34700: f64, t34703: f64, t34704: f64, t34708: f64, t34711: f64, t34713: f64, t34716: f64, t34718: f64, t34722: f64, t34724: f64, t34728: f64, t34732: f64) -> f64 {
    let t34736 = t8463 * t1181 * t604 * t5265;
    let t34738 = t30792 * t2264;
    let t34740 = t8511 * t4567;
    let t34742 = 0.13753125e0_f64 * t34694 - 0.21437009059034868486e-2_f64 * t34698 + 0.17149607247227894789e-2_f64 * t34700 - t34703 - 0.77173232612525526551e-2_f64 * t34704 + 0.10718504529517434243e-2_f64 * t34708 + t34711 + t34713 - 0.25724410870841842183e-2_f64 * t34716 - 0.25724410870841842184e-2_f64 * t34718 + 0.15724046144802076034e-2_f64 * t34722 + 0.94344276868812456204e-2_f64 * t34724 + 0.14151641530321868431e-1_f64 * t34728 - 0.94344276868812456204e-2_f64 * t34732 - 0.64311027177104605458e-2_f64 * t34736 + 0.12862205435420921092e-2_f64 * t34738 + 0.85748036236139473944e-3_f64 * t34740 - t30715;
    t34742
}
