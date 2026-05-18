//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1084/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1084<F: Float>(t11524: F, t11526: F, t11529: F, t11533: F, t11535: F, t11537: F, t11538: F, t11541: F, t11543: F, t11546: F, t11548: F, t11552: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39163 = t11524 / F::new(2.0);
    let t39164 = t11526 / F::new(2.0);
    let t39167 = F::new(5.0) / F::new(8.0) * t11529;
    let t39168 = F::new(5.0) / F::new(8.0) * t11533;
    let t39169 = F::new(2.0) * t11535;
    let t39170 = F::new(2.0) * t11537;
    let t39171 = F::new(2.0) * t11538;
    let t39172 = t11541 / F::new(2.0);
    let t39173 = F::new(2.0) * t11543;
    let t39174 = F::new(5.0) / F::new(8.0) * t11546;
    let t39175 = t11548 / F::new(2.0);
    let t39176 = F::new(3.0) / F::new(2.0) * t11552;
    (t39163, t39164, t39167, t39168, t39169, t39170, t39171, t39172, t39173, t39174, t39175, t39176)
}
