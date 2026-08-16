//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 712/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk712(t3701: f64, t988: f64, t11977: f64, t993: f64, t14266: f64, t169: f64, t172: f64, t452: f64, t105: f64, t13306: f64, t13309: f64, t13312: f64, t13315: f64, t13321: f64, t13329: f64, t13330: f64, t13726: f64, t2268: f64) -> (f64, f64, f64, f64, f64) {
    let t14277 = t3701 * t988;
    let t14280 = t11977 * t993;
    let t14284 = t14266 * t169 * t172;
    let t14285 = t452 * t14284;
    let t14288 = 0.47425011059460249332e-2_f64 * t13726 + t13306 - t13309 + t13312 - t13315 + t13321 + 0.56910013271352299198e-1_f64 * t2268 * t14277 - 0.1707300398140568976e0_f64 * t2268 * t14280 + 0.28455006635676149599e-1_f64 * t105 * t14285 + t13329 - t13330;
    (t14277, t14280, t14284, t14285, t14288)
}
