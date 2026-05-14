//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 971/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk971<F: Float>(t11513: F, t11516: F, t11520: F, t11524: F, t11526: F, t11529: F, t11533: F, t11535: F, t11537: F, t11538: F, t11541: F, t11543: F, t11546: F, t11548: F, t11552: F, t11557: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39160 = 2.0 * t11513;
    let t39161 = t11516 / 2.0;
    let t39162 = 15.0 / 8.0 * t11520;
    let t39163 = t11524 / 2.0;
    let t39164 = t11526 / 2.0;
    let t39167 = 5.0 / 8.0 * t11529;
    let t39168 = 5.0 / 8.0 * t11533;
    let t39169 = 2.0 * t11535;
    let t39170 = 2.0 * t11537;
    let t39171 = 2.0 * t11538;
    let t39172 = t11541 / 2.0;
    let t39173 = 2.0 * t11543;
    let t39174 = 5.0 / 8.0 * t11546;
    let t39175 = t11548 / 2.0;
    let t39176 = 3.0 / 2.0 * t11552;
    let t39177 = 5.0 / 8.0 * t11557;
    (t39160, t39161, t39162, t39163, t39164, t39167, t39168, t39169, t39170, t39171, t39172, t39173, t39174, t39175, t39176, t39177)
}
