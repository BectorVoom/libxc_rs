//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 219/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk219<F: Float>(t687: F, t690: F, t251: F, t258: F, t266: F, t273: F, t4: F, t668: F, t71: F, t721: F, t722: F, t728: F, t730: F, t740: F, t745: F, t748: F, t753: F, t757: F, t764: F, t774: F, t782: F, t786: F, t792: F, t793: F, t796: F, t799: F, t84: F) -> (F, F) {
    let t800 = t687 * t690;
    let t803 = -0.70983522622222222221e-3 * t4 * t668 * t71 - 0.34246666666666666666e-1 * t721 * t722 * t258 - 2.0 * t728 * t730 + 1.0 * t251 * t740 + 0.32163958997385070134e2 * t745 * t748 + t753 + t757 + t764 - t774 - t782 - 0.24415263074675393405e-3 * t4 * t668 * t84 - 0.10843581300301739842e-1 * t721 * t786 * t273 - 0.11696447245269292414e1 * t792 * t793 + 0.5848223622634646207e0 * t266 * t796 + 0.17315859105681463759e2 * t799 * t800;
    (t800, t803)
}
