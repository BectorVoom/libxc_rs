//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 889/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk889<F: Float>(t3132: F, t3175: F, t979: F, t1002: F, t12705: F, t3077: F, t3133: F, t3176: F, t116: F, t12699: F, t12698: F, t12809: F, t142: F, t140: F, t191: F, t918: F, t974: F) -> (F, F, F, F, F, F, F, F) {
    let t15175 = t3132 * t3175;
    let t15176 = t979 * t15175;
    let t15178 = t12705 * t1002;
    let t15179 = t979 * t15178;
    let t15181 = t3077 * t3133;
    let t15183 = t3077 * t3176;
    let t15185 = t116 * t12699;
    let t15186 = t12698 * t15185;
    let t15187 = t979 * t15186;
    let t15189 = t142 * t12809;
    let t15191 = t140 * t15189 * t191;
    let t15193 = t918 * t974;
    (t15176, t15179, t15181, t15183, t15187, t15189, t15191, t15193)
}
