//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 954/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk954<F: Float>(t1165: F, t5265: F, t7351: F, t8463: F, t1181: F, t5106: F, t604: F, t2264: F, t30792: F, t4567: F, t8511: F, t30715: F, t34694: F, t34698: F, t34700: F, t34703: F, t34704: F, t34708: F, t34711: F, t34713: F, t34716: F, t34718: F, t34722: F, t34724: F) -> (F,) {
    let t34728 = t8463 * t1165 * t7351 * t5265;
    let t34732 = t8463 * t1181 * t604 * t5106;
    let t34736 = t8463 * t1181 * t604 * t5265;
    let t34738 = t30792 * t2264;
    let t34740 = t8511 * t4567;
    let t34742 = 0.13753125e0 * t34694 - 0.21437009059034868486e-2 * t34698 + 0.17149607247227894789e-2 * t34700 - t34703 - 0.77173232612525526551e-2 * t34704 + 0.10718504529517434243e-2 * t34708 + t34711 + t34713 - 0.25724410870841842183e-2 * t34716 - 0.25724410870841842184e-2 * t34718 + 0.15724046144802076034e-2 * t34722 + 0.94344276868812456204e-2 * t34724 + 0.14151641530321868431e-1 * t34728 - 0.94344276868812456204e-2 * t34732 - 0.64311027177104605458e-2 * t34736 + 0.12862205435420921092e-2 * t34738 + 0.85748036236139473944e-3 * t34740 - t30715;
    (t34742,)
}
