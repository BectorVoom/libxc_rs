//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 219/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk219<F: Float>(t257: F, t739: F, t249: F, t62: F, t70: F, t729: F, t31: F, t4: F, t668: F, t132: F, t200: F, t220: F, t721: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t740 = t739 * t257;
    let t743 = t249 * t249;
    let t744 = F::new(1.0) / t743;
    let t745 = t62 * t744;
    let t746 = t70 * t70;
    let t747 = F::new(1.0) / t746;
    let t748 = t729 * t747;
    let t752 = t4 * t668 * t31;
    let t753 = F::new(0.14764627977777777777e-2) * t752;
    let t754 = t132 * t200;
    let t756 = t721 * t754 * t220;
    (t740, t743, t744, t745, t746, t747, t748, t753, t754, t756)
}
