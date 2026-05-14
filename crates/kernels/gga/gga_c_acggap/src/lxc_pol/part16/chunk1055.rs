//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1055/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1055<F: Float>(t13299: F, t33944: F, t40017: F, t13287: F, t31195: F, t39827: F, t17912: F, t31443: F, t39854: F, t31797: F, t31806: F, t36163: F, t36176: F, t36178: F, t36195: F, t36199: F, t36206: F, t36208: F, t36211: F, t36215: F, t36240: F, t37918: F, t37922: F, t37923: F) -> (F,) {
    let t40450 = t33944 * t13299 * t40017;
    let t40455 = t31195 * t13287 * t39827;
    let t40458 = t31443 * t17912 * t39854;
    let t40460 = t36163 - t36176 - t36178 - t36195 + t36199 - t36206 - t36208 - t36211 - t36215 + 0.94344276868812456204e-2 * t40450 - 0.15724046144802076034e-3 * t31797 - t31806 - t37918 + t37922 - t37923 - 0.80031500487063509015e-2 * t36240 - 0.21437009059034868486e-2 * t40455 - 0.12862205435420921092e-2 * t40458;
    (t40460,)
}
