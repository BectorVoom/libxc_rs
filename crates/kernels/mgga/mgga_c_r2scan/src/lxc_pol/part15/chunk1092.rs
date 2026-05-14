//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1092/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1092<F: Float>(t1100: F, t11862: F, t354: F, t39181: F, t39186: F, t39188: F, t39195: F, t39201: F, t39205: F, t39208: F, t39212: F, t39246: F, t39267: F, t39271: F, t39272: F, t39276: F, t39278: F, t39282: F, t39322: F, t40272: F, t40321: F, t40355: F, t40401: F, t40437: F, t40476: F, t40506: F, t40533: F, t40583: F, t40616: F, t40655: F, t40695: F, t40718: F, t8306: F, t860: F) -> (F,) {
    let t40724 = t39181 + t39186 + t39188 + t39195 - t39201 - t39205 + t1100 * t8306 - t39208 + t39212 + 2.0 * t860 * t11862 + t354 * (t39246 + t39272 + t39322 + t40272 + t40321 + t40355 + t40401 + t40437 + t40476 + t40506 + t40533 + t40583 + t40616 + t40655 + t40695 + t40718) - t39267 + t39271 + t39276 - t39278 + t39282;
    (t40724,)
}
