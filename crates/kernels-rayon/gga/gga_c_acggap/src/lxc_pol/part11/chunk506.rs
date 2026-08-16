//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 506/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk506(t13: f64, t2808: f64, t2666: f64, t779: f64, t272: f64, t2775: f64, t132: f64, t251: f64, t256: f64, t258: f64, t266: f64, t2723: f64, t2737: f64, t2738: f64, t2743: f64, t2755: f64, t2760: f64, t2763: f64, t2768: f64, t2769: f64, t2774: f64, t2776: f64, t2788: f64, t2793: f64, t2796: f64, t2806: f64, t721: f64, t722: f64, t727: f64, t728: f64, t730: f64, t739: f64, t740: f64, t745: f64, t747: f64, t793: f64, t799: f64) -> (f64, f64) {
    let t2809 = t13 * t2808;
    let t2810 = t2666 * t779;
    let t2811 = t2809 * t2810;
    let t2812 = 0.96491876992155210402e2_f64 * t2811;
    let t2813 = t2775 * t272;
    let t2823 = 0.68493333333333333332e-1_f64 * t721 * t2723 * t258 - 0.51369999999999999999e-1_f64 * t721 * t722 * t740 + 0.10274e0_f64 * t721 * t132 * t727 * t730 - t2737 + 0.32530743900905219526e-1_f64 * t721 * t2738 * t793 + 6.0_f64 * t745 * t2743 + 1.0_f64 * t251 * t2755 + 0.2069040516770936012e4_f64 * t2760 * t2763 - 0.19298375398431042081e3_f64 * t2768 * t2769 - 0.10389515463408878255e3_f64 * t2774 * t2776 + 0.5848223622634646207e0_f64 * t266 * t2788 + 0.10254018858216406658e4_f64 * t2793 * t2796 - t2806 + t2812 + 0.35089341735807877242e1_f64 * t799 * t2813 + 0.96491876992155210402e2_f64 * t745 * t739 * t747 * t256 - 6.0_f64 * t728 * t258 * t739;
    (t2812, t2823)
}
