//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 418/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk418<F: Float>(t222: F, t224: F, t3277: F, t3278: F, t3283: F, t229: F, t1060: F, zeta_threshold: F) -> (F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t3287 = piecewise3(t223, 0.0, 4.0 / 9.0 * t3277 * t3278 + 4.0 / 3.0 * t224 * t3283);
    let t3288 = t229 * t229;
    let t3289 = 1.0 / t3288;
    let t3290 = t1060 * t1060;
    (t3287, t3288, t3289, t3290)
}
