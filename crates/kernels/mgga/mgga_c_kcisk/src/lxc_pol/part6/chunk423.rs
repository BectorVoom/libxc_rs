//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 423/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk423<F: Float>(t3234: F, t3237: F, t3239: F, t3243: F, t3246: F, t3249: F, t3251: F, t3254: F, t3256: F, t3258: F, t3261: F, t3264: F, t3267: F, t3272: F, t1010: F, t224: F) -> (F, F, F) {
    let t3274 = t3234 / 8.0 - t3237 / 4.0 - t3239 / 2.0 + t3243 / 4.0 + t3246 / 2.0 - t3249 / 8.0 + 3.0 / 4.0 * t3251 - t3254 / 64.0 + t3256 / 32.0 + t3258 / 8.0 - t3261 / 32.0 - t3264 / 8.0 + t3267 / 64.0 - 5.0 / 16.0 * t3272;
    let t3275 = t1010 * t3274;
    let t3276 = t224 * t224;
    (t3274, t3275, t3276)
}
