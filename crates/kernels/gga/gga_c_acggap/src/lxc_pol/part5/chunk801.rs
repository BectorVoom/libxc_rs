//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 801/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk801<F: Float>(t739: F, t207: F, t218: F, t771: F, t776: F, t779: F, t759: F, t772: F, t11649: F, t11805: F, t11870: F, t11909: F, t11914: F, t11934: F, t11944: F, t256: F, t257: F, t2627: F, t2632: F, t266: F, t271: F, t272: F, t2760: F, t2762: F, t2774: F, t2787: F, t2793: F, t4: F, t680: F, t690: F, t71: F, t727: F, t728: F, t729: F, t730: F, t740: F, t744: F, t745: F, t747: F, t799: F) -> (F, F, F) {
    let t12098 = t739 * t739;
    let t12126 = 0.34367190188705947438e1 * t207 * t776 * t771 * t779 * t218;
    let t12130 = 0.4274e0 * t207 * t759 * t218 * t772;
    let t12144 = -0.55209406483950617283e-2 * t4 * t11805 * t71 + 36.0 * t745 * t730 * t739 - 6.0 * t728 * t12098 * t257 + 0.96491876992155210402e2 * t745 * t12098 * t747 + t11909 + t11914 + 0.5848223622634646207e0 * t266 * t11934 * t272 - 0.14035736694323150897e2 * t2774 * t11870 * t272 + t11944 + 0.12414243100625616072e5 * t2760 * t729 * t2762 * t739 + 0.69263436422725855036e2 * t799 * t2787 * t690 * t271 + 0.61524113149298439947e4 * t2793 * t11649 * t680 + t12126 - t12130 - 0.19263893255070628432e1 * t207 * t2632 + 0.1301229756036208781e0 * t207 * t2627 - 0.6609050294782684211e1 * t207 * t744 * t739 * t747 * t256 + 0.41096e0 * t207 * t727 * t256 * t740;
    (t12126, t12130, t12144)
}
