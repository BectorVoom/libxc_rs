//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 696/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk696<F: Float>(t2049: F, t607: F, t759: F, t4733: F, t4736: F, t4739: F, t5860: F, t166: F, t122: F, t1415: F, t2111: F, t2117: F, t57: F, t1605: F, t537: F, t481: F, t495: F) -> (F, F, F, F, F, F) {
    let t6038 = t607 * t2049;
    let t6039 = t759 * t6038;
    let t6044 = -0.29633333333333333333e-1 * t4733 + 0.19755555555555555555e-1 * t4736 - 0.23048148148148148148e-1 * t4739 - t5860;
    let t6045 = t166 * t6044;
    let t6047 = 0.285764e-1 * t759 * t6045;
    let t6062 = 0.1590300183910403919e-2 * t2111 * t122 * t1415 * t57 * t2117;
    let t6063 = t1605 * t537;
    let t6064 = t495 * t481;
    (t6039, t6044, t6047, t6062, t6063, t6064)
}
