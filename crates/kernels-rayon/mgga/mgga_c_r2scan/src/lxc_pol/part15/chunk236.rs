//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 236/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk236(t695: f64, t721: f64, t201: f64, t202: f64, t207: f64, t208: f64, t220: f64, t226: f64, t624: f64, t625: f64, t629: f64, t635: f64, t647: f64, t656: f64, t664: f64, t668: f64, t674: f64, t682: f64, t687: f64, t690: f64, t697: f64, t699: f64, t705: f64, t706: f64, t713: f64, t718: f64) -> (f64, f64) {
    let t722 = t721 * t695;
    let t725 = 0.20548e0_f64 * t201 * t664 * t207 - 0.17123333333333333333e-1_f64 * t625 * t668 * t208 - 2.0_f64 * t674 * t208 * t664 + 1.0_f64 * t202 * t682 + 0.32163958997385070134e2_f64 * t687 * t690 * t664 - t624 + t629 + t635 - t647 - t656 + 0.65061487801810439052e-1_f64 * t697 - 0.54217906501508699211e-2_f64 * t625 * t699 * t226 - 0.11696447245269292414e1_f64 * t705 * t706 + 0.5848223622634646207e0_f64 * t220 * t713 + 0.17315859105681463759e2_f64 * t718 * t722;
    (t722, t725)
}
