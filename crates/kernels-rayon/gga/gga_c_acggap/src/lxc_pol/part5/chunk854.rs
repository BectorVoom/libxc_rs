//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 854/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk854(t739: f64, t207: f64, t218: f64, t771: f64, t776: f64, t779: f64, t759: f64, t772: f64, t11649: f64, t11805: f64, t11870: f64, t11909: f64, t11914: f64, t11934: f64, t11944: f64, t256: f64, t257: f64, t2627: f64, t2632: f64, t266: f64, t271: f64, t272: f64, t2760: f64, t2762: f64, t2774: f64, t2787: f64, t2793: f64, t4: f64, t680: f64, t690: f64, t71: f64, t727: f64, t728: f64, t729: f64, t730: f64, t740: f64, t744: f64, t745: f64, t747: f64, t799: f64) -> (f64, f64, f64) {
    let t12098 = t739 * t739;
    let t12126 = 0.34367190188705947438e1_f64 * t207 * t776 * t771 * t779 * t218;
    let t12130 = 0.4274e0_f64 * t207 * t759 * t218 * t772;
    let t12144 = -0.55209406483950617283e-2_f64 * t4 * t11805 * t71 + 36.0_f64 * t745 * t730 * t739 - 6.0_f64 * t728 * t12098 * t257 + 0.96491876992155210402e2_f64 * t745 * t12098 * t747 + t11909 + t11914 + 0.5848223622634646207e0_f64 * t266 * t11934 * t272 - 0.14035736694323150897e2_f64 * t2774 * t11870 * t272 + t11944 + 0.12414243100625616072e5_f64 * t2760 * t729 * t2762 * t739 + 0.69263436422725855036e2_f64 * t799 * t2787 * t690 * t271 + 0.61524113149298439947e4_f64 * t2793 * t11649 * t680 + t12126 - t12130 - 0.19263893255070628432e1_f64 * t207 * t2632 + 0.1301229756036208781e0_f64 * t207 * t2627 - 0.6609050294782684211e1_f64 * t207 * t744 * t739 * t747 * t256 + 0.41096e0_f64 * t207 * t727 * t256 * t740;
    (t12126, t12130, t12144)
}
