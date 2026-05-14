//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 123/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk123<F: Float>(t155: F, t389: F, t163: F, t158: F, t247: F, t250: F, t369: F, t374: F, t166: F) -> (F, F, F, F, F, F) {
    let t390 = t155 * t389;
    let t394 = t163 * t163;
    let t395 = 1.0 / t394;
    let t396 = t158 * t395;
    let t401 = -0.1176575e1 * t247 - 0.516475e0 * t250 - 0.2103875e0 * t369 - 0.104195e0 * t374;
    let t402 = 1.0 / t166;
    (t390, t394, t395, t396, t401, t402)
}
