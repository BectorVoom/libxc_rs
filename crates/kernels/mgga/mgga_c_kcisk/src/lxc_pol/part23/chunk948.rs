//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 948/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk948<F: Float>(t19240: F, t3544: F, t425: F, t5926: F, t1364: F, t5662: F, t1390: F, t2083: F, t3278: F, t3539: F, t3521: F, t5923: F, t442: F, t5684: F, t1056: F, t3283: F, t5921: F) -> (F, F, F, F, F, F) {
    let t19241 = t3544 * t19240;
    let t19244 = t5926 * t425;
    let t19246 = t19244 * t5662 * t1364;
    let t19251 = t3539 * t2083 * t1390 * t3278;
    let t19255 = 0.13140859333333333333e-2 * t3521 * t5923;
    let t19256 = t5684 * t442;
    let t19257 = t19256 * t1056;
    let t19258 = t3539 * t19257;
    let t19262 = t3539 * t5921 * t3283;
    (t19241, t19246, t19251, t19255, t19258, t19262)
}
