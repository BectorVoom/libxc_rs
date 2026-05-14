//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 653/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk653<F: Float>(t12453: F, t12649: F, t190: F, t207: F, t206: F, t9355: F, t1039: F, t3233: F, t116: F, t3241: F, t3174: F, t9345: F, t211: F, t3138: F, t1001: F, t213: F) -> (F, F, F, F, F, F) {
    let t12650 = t12453 + t12649;
    let t12651 = t12650 * t190;
    let t12652 = t12651 * t207;
    let t12654 = t206 * t9355;
    let t12656 = t3233 * t1039;
    let t12658 = t3241 * t116;
    let t12659 = t9345 * t3174;
    let t12660 = t12658 * t12659;
    let t12662 = t3138 * t211;
    let t12663 = t213 * t1001;
    (t12652, t12654, t12656, t12660, t12662, t12663)
}
