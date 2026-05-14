//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1005/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1005<F: Float>(t23107: F, t23274: F, t673: F, t716: F, t720: F, t415: F, t4811: F, t8948: F, t10517: F, t16673: F, t16688: F, t23045: F, t23048: F, t23055: F, t23058: F, t4830: F, t7275: F, t7278: F, t8852: F) -> (F, F, F, F, F) {
    let t23275 = t23107 + t23274;
    let t23276 = t673 * t23275;
    let t23277 = t23276 * t716;
    let t23278 = t23277 * t720;
    let t23279 = t415 * t23278;
    let t23286 = t4811 * t8948;
    let t23288 = -0.88437037037037037034e-2 * t23045 + 0.16581944444444444444e-2 * t23048 + 0.16581944444444444444e-2 * t23055 - 0.49745833333333333332e-2 * t23058 + 0.24872916666666666666e-2 * t23279 + t16673 - 0.386e0 * t7278 * t7275 + 0.193e0 * t4830 * t8852 - 0.55273148148148148147e-3 * t10517 + t16688 - 0.22109259259259259259e-2 * t23286;
    (t23275, t23276, t23279, t23286, t23288)
}
