//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 90/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk90<F: Float>(t273: F, t6: F, t123: F, t119: F, t268: F, t193: F, t259: F, t208: F, t215: F, t5: F, t7: F) -> (F, F, F, F, F, F, F, F, F) {
    let t274 = t6 * t273;
    let t275 = t123 * t274;
    let t278 = 1.0 + 0.5397236614853195164e-1 * t268 * t119 * t275;
    let t279 = f64::ln(t278);
    let t281 = 1.0 + 0.193e0 * t279;
    let t282 = 1.0 / t281;
    let t285 = t259 * t282 + 0.69644166666666666665e-2 * t193;
    let t288 = 1.0 + 0.1875e0 * t208 - 0.4046875e-1 * t215;
    let t289 = 1.0 / t288;
    let t298 = t5 * t7;
    (t274, t275, t278, t281, t282, t285, t288, t289, t298)
}
