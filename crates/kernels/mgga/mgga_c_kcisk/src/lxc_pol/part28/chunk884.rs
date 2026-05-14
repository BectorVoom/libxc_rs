//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 884/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk884<F: Float>(t15422: F, t933: F, t116: F, t12769: F, t982: F, t979: F, t119: F, t3127: F, t140: F, t191: F, t1002: F, t3174: F, t3138: F, t2925: F, t855: F, t12630: F, t136: F, t15203: F, t15217: F, t15221: F, t15226: F, t2927: F, t2932: F, t2936: F, t3064: F, t856: F, t934: F) -> (F, F, F, F, F, F) {
    let t15423 = t15422 * t933;
    let t15426 = t116 * t12769;
    let t15427 = t982 * t15426;
    let t15428 = t979 * t15427;
    let t15430 = t119 * t3127;
    let t15432 = t140 * t15430 * t191;
    let t15434 = t1002 * t3174;
    let t15435 = t3138 * t15434;
    let t15436 = t979 * t15435;
    let t15445 = t2925 * t855;
    let t15450 = 0.223494e0 * t15217 * t2936 + 0.223494e0 * t2932 * t15221 - 0.10317654320987654321e0 * t15226 - 0.193e0 * t856 * t15423 - 0.99491666666666666664e-2 * t15428 - 0.39796666666666666665e-1 * t15432 + 0.59694999999999999999e-1 * t15436 - 0.579e0 * t2927 * t3064 + t12630 * t136 + 0.579e0 * t856 * t15221 + 0.579e0 * t2927 * t2936 - 0.579e0 * t15445 * t934 - 0.386e0 * t856 * t15203;
    (t15428, t15430, t15432, t15436, t15445, t15450)
}
