//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 83/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk83(t259: f64, t6: f64, t123: f64, t119: f64, t268: f64, t193: f64, t208: f64, t215: f64, t219: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t270 = 0.0_f64 < t259;
    let t272 = piecewise3(t270, t259, -t259);
    let t273 = 1.0_f64 / t272;
    let t274 = t6 * t273;
    let t275 = t123 * t274;
    let t278 = 1.0_f64 + 0.5397236614853195164e-1_f64 * t268 * t119 * t275;
    let t279 = f64::ln(t278);
    let t281 = 1.0_f64 + 0.193e0_f64 * t279;
    let t282 = 1.0_f64 / t281;
    let t285 = t259 * t282 + 0.69644166666666666665e-2_f64 * t193;
    let t288 = 1.0_f64 + 0.1875e0_f64 * t208 - 0.4046875e-1_f64 * t215;
    let t289 = 1.0_f64 / t288;
    let t291 = t285 * t289 - t219;
    let t293 = 1.0_f64 / rho0;
    let t294 = sigma0 * t293;
    (t272, t275, t278, t281, t282, t285, t288, t289, t291, t294)
}
