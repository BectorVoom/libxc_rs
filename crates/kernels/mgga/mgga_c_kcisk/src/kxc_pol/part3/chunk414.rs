//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 414/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk414<F: Float>(t190: F, t3127: F, t214: F, t1045: F, t3132: F, t1042: F, t1050: F, t3139: F, t3138: F, t1001: F, t982: F, t3174: F, t117: F, t212: F, t211: F, t210: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3253 = t3127 * t190;
    let t3254 = t3253 * t214;
    let t3256 = t3132 * t1045;
    let t3258 = t1042 * t1050;
    let t3260 = t214 * t3139;
    let t3261 = t3138 * t3260;
    let t3263 = t1050 * t1001;
    let t3264 = t982 * t3263;
    let t3266 = t214 * t3174;
    let t3267 = t982 * t3266;
    let t3269 = t212 * t117;
    let t3270 = 1.0 / t3269;
    let t3271 = t211 * t3270;
    let t3272 = t210 * t3271;
    (t3253, t3254, t3256, t3258, t3260, t3261, t3263, t3264, t3266, t3267, t3271, t3272)
}
