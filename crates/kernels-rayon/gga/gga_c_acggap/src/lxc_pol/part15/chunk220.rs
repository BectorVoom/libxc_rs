//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 220/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk220(t272: f64, t687: f64, t680: f64, t686: f64, t75: f64, t690: f64, t251: f64, t258: f64, t266: f64, t273: f64, t4: f64, t668: f64, t71: f64, t721: f64, t722: f64, t728: f64, t730: f64, t740: f64, t745: f64, t748: f64, t753: f64, t757: f64, t764: f64, t774: f64, t782: f64, t786: f64, t792: f64, t84: f64) -> (f64, f64, f64, f64, f64) {
    let t793 = t687 * t272;
    let t796 = t680 * t272;
    let t799 = t75 * t686;
    let t800 = t687 * t690;
    let t803 = -0.70983522622222222221e-3_f64 * t4 * t668 * t71 - 0.34246666666666666666e-1_f64 * t721 * t722 * t258 - 2.0_f64 * t728 * t730 + 1.0_f64 * t251 * t740 + 0.32163958997385070134e2_f64 * t745 * t748 + t753 + t757 + t764 - t774 - t782 - 0.24415263074675393405e-3_f64 * t4 * t668 * t84 - 0.10843581300301739842e-1_f64 * t721 * t786 * t273 - 0.11696447245269292414e1_f64 * t792 * t793 + 0.5848223622634646207e0_f64 * t266 * t796 + 0.17315859105681463759e2_f64 * t799 * t800;
    (t793, t796, t799, t800, t803)
}
