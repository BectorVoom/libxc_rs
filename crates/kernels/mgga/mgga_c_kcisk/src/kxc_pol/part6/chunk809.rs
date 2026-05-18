//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 809/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk809<F: Float>(t4265: F, t8995: F, t140: F, t299: F, t9010: F, t9003: F, t9007: F, t695: F, t8662: F, t22249: F, t740: F, t5439: F, t9234: F) -> (F, F, F, F, F, F, F) {
    let t24320 = t4265 * t8995;
    let t24324 = t140 * t299 * t9010;
    let t24374 = t4265 * t9003;
    let t24376 = t4265 * t9007;
    let t24434 = t8662 * t695;
    let t24473 = t22249 * t740;
    let t24561 = t9234 * t5439;
    (t24320, t24324, t24374, t24376, t24434, t24473, t24561)
}
