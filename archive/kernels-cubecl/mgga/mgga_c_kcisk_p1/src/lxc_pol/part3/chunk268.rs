//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 268/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk268<F: Float>(t1266: F, t1273: F, t1275: F, t1229: F, t1234: F, t1255: F, t1260: F, t1264: F, t361: F, t374: F, t45: F, t67: F) -> (F, F) {
    let t1276 = t1266 * t1273 * t1275;
    let t1279 = -F::cast_from(0.62182e-1_f64) * t1229 * t361 + F::cast_from(1.0_f64) * t1234 * t1255 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t1260 * t374 - F::cast_from(0.58482233974552040708e0_f64) * t1264 * t1276;
    let t1280 = t67 * t1279;
    (t1276, t1280)
}
