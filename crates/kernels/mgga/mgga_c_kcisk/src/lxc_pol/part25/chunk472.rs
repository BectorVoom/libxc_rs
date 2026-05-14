//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 472/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk472<F: Float>(t222: F, t224: F, t1056: F, t220: F, t967: F, t167: F, t229: F, t1060: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t3276 = t224 * t224;
    let t3277 = 1.0 / t3276;
    let t3278 = t1056 * t1056;
    let t3281 = t220 * t967;
    let t3283 = -2.0 * t167 + 2.0 * t3281;
    let t3287 = piecewise3(t223, 0.0, 4.0 / 9.0 * t3277 * t3278 + 4.0 / 3.0 * t224 * t3283);
    let t3288 = t229 * t229;
    let t3289 = 1.0 / t3288;
    let t3290 = t1060 * t1060;
    (t3276, t3277, t3278, t3281, t3283, t3287, t3288, t3289, t3290)
}
