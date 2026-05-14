//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 491/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk491<F: Float>(t2775: F, t2795: F, t199: F, t775: F, t13: F, t30: F, t778: F, t2666: F, t27: F, t779: F, t272: F, t132: F, t251: F, t256: F, t258: F, t266: F, t2723: F, t2737: F, t2738: F, t2743: F, t2755: F, t2760: F, t2763: F, t2768: F, t2769: F, t2774: F, t2776: F, t2788: F, t2793: F, t721: F, t722: F, t727: F, t728: F, t730: F, t739: F, t740: F, t745: F, t747: F, t793: F, t799: F) -> (F, F, F) {
    let t2796 = t2775 * t2795;
    let t2800 = 1.0 / t775 / t199;
    let t2801 = t13 * t2800;
    let t2803 = 1.0 / t778 / t30;
    let t2804 = t2666 * t2803;
    let t2805 = t2801 * t2804;
    let t2806 = 0.51726012919273400301e3 * t2805;
    let t2808 = 1.0 / t775 / t27;
    let t2809 = t13 * t2808;
    let t2810 = t2666 * t779;
    let t2811 = t2809 * t2810;
    let t2812 = 0.96491876992155210402e2 * t2811;
    let t2813 = t2775 * t272;
    let t2823 = 0.68493333333333333332e-1 * t721 * t2723 * t258 - 0.51369999999999999999e-1 * t721 * t722 * t740 + 0.10274e0 * t721 * t132 * t727 * t730 - t2737 + 0.32530743900905219526e-1 * t721 * t2738 * t793 + 6.0 * t745 * t2743 + 1.0 * t251 * t2755 + 0.2069040516770936012e4 * t2760 * t2763 - 0.19298375398431042081e3 * t2768 * t2769 - 0.10389515463408878255e3 * t2774 * t2776 + 0.5848223622634646207e0 * t266 * t2788 + 0.10254018858216406658e4 * t2793 * t2796 - t2806 + t2812 + 0.35089341735807877242e1 * t799 * t2813 + 0.96491876992155210402e2 * t745 * t739 * t747 * t256 - 6.0 * t728 * t258 * t739;
    (t2806, t2812, t2823)
}
