//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 234/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk234<F: Float>(t695: F, t721: F, t201: F, t202: F, t207: F, t208: F, t220: F, t226: F, t624: F, t625: F, t629: F, t635: F, t647: F, t656: F, t664: F, t668: F, t674: F, t682: F, t687: F, t690: F, t697: F, t699: F, t705: F, t706: F, t713: F, t718: F) -> (F, F) {
    let t722 = t721 * t695;
    let t725 = 0.20548e0 * t201 * t664 * t207 - 0.17123333333333333333e-1 * t625 * t668 * t208 - 2.0 * t674 * t208 * t664 + 1.0 * t202 * t682 + 0.32163958997385070134e2 * t687 * t690 * t664 - t624 + t629 + t635 - t647 - t656 + 0.65061487801810439052e-1 * t697 - 0.54217906501508699211e-2 * t625 * t699 * t226 - 0.11696447245269292414e1 * t705 * t706 + 0.5848223622634646207e0 * t220 * t713 + 0.17315859105681463759e2 * t718 * t722;
    (t722, t725)
}
