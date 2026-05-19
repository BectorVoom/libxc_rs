//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 712/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk712<F: Float>(t3701: F, t988: F, t11977: F, t993: F, t14266: F, t169: F, t172: F, t452: F, t105: F, t13306: F, t13309: F, t13312: F, t13315: F, t13321: F, t13329: F, t13330: F, t13726: F, t2268: F) -> (F, F, F, F, F) {
    let t14277 = t3701 * t988;
    let t14280 = t11977 * t993;
    let t14284 = t14266 * t169 * t172;
    let t14285 = t452 * t14284;
    let t14288 = F::cast_from(0.47425011059460249332e-2_f64) * t13726 + t13306 - t13309 + t13312 - t13315 + t13321 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t14277 - F::cast_from(0.1707300398140568976e0_f64) * t2268 * t14280 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t14285 + t13329 - t13330;
    (t14277, t14280, t14284, t14285, t14288)
}
