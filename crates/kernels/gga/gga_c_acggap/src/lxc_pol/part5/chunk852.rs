//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 852/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk852<F: Float>(t729: F, t11582: F, t11586: F, t11708: F, t11735: F, t11795: F, t11797: F, t11800: F, t11803: F, t11805: F, t11806: F, t11811: F, t11813: F, t11815: F, t11817: F, t11820: F, t11870: F, t11948: F, t11951: F, t123: F, t132: F, t250: F, t251: F, t257: F, t258: F, t271: F, t272: F, t2760: F, t2762: F, t2774: F, t2788: F, t2792: F, t2793: F, t2796: F, t328: F, t4: F, t62: F, t680: F, t690: F, t721: F, t726: F, t727: F, t730: F, t743: F, t744: F, t747: F, t748: F, t75: F, t786: F, t792: F, t800: F, t84: F) -> (F, F) {
    let t12015 = t729 * t729;
    let t12058 = t11582 + t11586 + F::cast_from(0.44060335298551228073e1_f64) * t721 * t123 * t744 * t748 - F::cast_from(0.21309037037037037036e0_f64) * t721 * t328 * t250 * t258 - F::cast_from(0.27397333333333333333e0_f64) * t721 * t123 * t727 * t730 - F::cast_from(0.21687162600603479684e-1_f64) * t721 * t786 * t2788 - F::cast_from(0.38025319932552508021e2_f64) * t721 * t132 * t2792 * t2796 + F::cast_from(0.11579025239058625248e4_f64) * t2760 * t12015 * t747 - F::cast_from(0.24828486201251232145e5_f64) * t62 / t743 / t726 * t12015 * t2762 + F::new(1.0) * t251 * (-F::cast_from(0.39219166666666666667e1_f64) * t11795 + F::new(0.376504e2) * t11797 - F::cast_from(0.13944592592592592593e2_f64) * t11800 + F::cast_from(0.12201518518518518519e2_f64) * t11803 + F::cast_from(0.5356037037037037037e1_f64) * t11806 + F::cast_from(0.14025833333333333333e0_f64) * t11811 - F::cast_from(0.22441333333333333332e1_f64) * t11813 + F::cast_from(0.24934814814814814815e1_f64) * t11815 + F::cast_from(0.21817962962962962963e1_f64) * t11817 + F::cast_from(0.16979925925925925926e1_f64) * t11820) * t257 + F::cast_from(0.91082604192152556044e5_f64) * t75 * t11948 * t11870 * t11951 - F::cast_from(0.62337092780453269531e3_f64) * t2774 * t800 * t680 - F::cast_from(0.18989649058080861537e-2_f64) * t4 * t11805 * t84 - F::cast_from(0.35089341735807877242e1_f64) * t792 * t11735 * t272 - F::cast_from(0.46785788981077169656e1_f64) * t792 * t2788 * t271 - t11708 + F::cast_from(0.6233709278045326953e3_f64) * t2793 * t11870 * t690;
    (t12015, t12058)
}
