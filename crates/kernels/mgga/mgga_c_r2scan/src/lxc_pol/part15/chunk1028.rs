//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1028/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1028<F: Float>(t24035: F, t3332: F, t6165: F, t11646: F, t22731: F, t11649: F, t25169: F, t37859: F, t37881: F, t39759: F, t39763: F, t39765: F, t39768: F, t39771: F, t39772: F, t39775: F) -> (F,) {
    let t39778 = t6165 * t3332 * t24035;
    let t39780 = t22731 * t11646;
    let t39782 = t25169 * t11649;
    let t39784 = 0.23115257973478049502e0 * t37859 + 0.47609969197673950972e-2 * t37881 - 0.5200933044032561138e0 * t39759 - t39763 - 0.2600466522016280569e0 * t39765 - 0.2600466522016280569e0 * t39768 + t39771 - 0.42683466926433871473e0 * t39772 - 0.87327386630866483584e-2 * t39775 - 0.13099107994629972538e-1 * t39778 - 0.13099107994629972538e-1 * t39780 - 0.5239643197851989015e-1 * t39782;
    (t39784,)
}
