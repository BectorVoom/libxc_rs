//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 437/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk437<F: Float>(t214: F, t3139: F, t3138: F, t1001: F, t1050: F, t982: F, t3174: F, t117: F, t212: F, t211: F, t210: F, t3234: F, t3237: F, t3239: F, t3243: F, t3246: F, t3249: F, t3251: F, t3254: F, t3256: F, t3258: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3260 = t214 * t3139;
    let t3261 = t3138 * t3260;
    let t3263 = t1050 * t1001;
    let t3264 = t982 * t3263;
    let t3266 = t214 * t3174;
    let t3267 = t982 * t3266;
    let t3269 = t212 * t117;
    let t3270 = F::cast_from(1.0_f64) / t3269;
    let t3271 = t211 * t3270;
    let t3272 = t210 * t3271;
    let t3274 = t3234 / F::cast_from(8.0_f64) - t3237 / F::cast_from(4.0_f64) - t3239 / F::cast_from(2.0_f64) + t3243 / F::cast_from(4.0_f64) + t3246 / F::cast_from(2.0_f64) - t3249 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3251 - t3254 / F::cast_from(64.0_f64) + t3256 / F::cast_from(32.0_f64) + t3258 / F::cast_from(8.0_f64) - t3261 / F::cast_from(32.0_f64) - t3264 / F::cast_from(8.0_f64) + t3267 / F::cast_from(64.0_f64) - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3272;
    (t3260, t3261, t3263, t3264, t3266, t3267, t3271, t3272, t3274)
}
