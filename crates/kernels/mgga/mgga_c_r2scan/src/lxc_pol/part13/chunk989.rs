//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 989/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk989<F: Float>(t11529: F, t11533: F, t11535: F, t11537: F, t11538: F, t11541: F, t11543: F, t11546: F, t11548: F, t11552: F, t11557: F, t11554: F, t2262: F, t3275: F, t3276: F, t6897: F, t910: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t39178 = t11554 * t2262;
    let t39181 = 5.0 / 16.0 * t3275 * t3276 * t39178;
    let t39182 = t6897 * t910;
    (t39167, t39168, t39169, t39170, t39171, t39172, t39173, t39174, t39175, t39176, t39177, t39181, t39182)
}
