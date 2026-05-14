//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 457/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk457<F: Float>(t123: F, t3058: F, t925: F, t121: F, t129: F, t3033: F, t3036: F, t3044: F, t3054: F, t913: F, t920: F, t929: F) -> (F, F) {
    let t3060 = t123 * t925 * t3058;
    let t3063 = 0.53972366148531951642e-1 * t3033 * t129 - 0.251871042026482441e0 * t3036 * t129 - 0.10794473229706390328e0 * t913 * t929 + 0.41978507004413740167e0 * t3044 * t129 + 0.251871042026482441e0 * t920 * t929 + 0.10794473229706390328e0 * t121 * t3054 - 0.53972366148531951642e-1 * t121 * t3060;
    (t3060, t3063)
}
