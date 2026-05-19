//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 83/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk83<F: Float>(t259: F, t6: F, t123: F, t119: F, t268: F, t193: F, t208: F, t215: F, t219: F, rho0: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t270 = F::new(0.0) < t259;
    let t272 = piecewise3::<F>(t270, t259, -t259);
    let t273 = F::new(1.0) / t272;
    let t274 = t6 * t273;
    let t275 = t123 * t274;
    let t278 = F::new(1.0) + F::cast_from(0.5397236614853195164e-1_f64) * t268 * t119 * t275;
    let t279 = F::ln(t278);
    let t281 = F::new(1.0) + F::new(0.193e0) * t279;
    let t282 = F::new(1.0) / t281;
    let t285 = t259 * t282 + F::cast_from(0.69644166666666666665e-2_f64) * t193;
    let t288 = F::new(1.0) + F::new(0.1875e0) * t208 - F::new(0.4046875e-1) * t215;
    let t289 = F::new(1.0) / t288;
    let t291 = t285 * t289 - t219;
    let t293 = F::new(1.0) / rho0;
    let t294 = sigma0 * t293;
    (t272, t275, t278, t281, t282, t285, t288, t289, t291, t294)
}
