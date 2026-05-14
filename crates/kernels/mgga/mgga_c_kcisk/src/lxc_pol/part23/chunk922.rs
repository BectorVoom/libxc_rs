//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 922/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk922<F: Float>(t1336: F, t140: F, t19053: F, t3800: F, t2266: F, t3583: F, t3484: F, t3482: F, t3575: F, t5633: F, t454: F, t2153: F, t3742: F, t3796: F, t13311: F, t6229: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19055 = t140 * t1336 * t19053;
    let t19056 = t19055 * t3800;
    let t19058 = t2266 * t3583;
    let t19059 = t3484 * t19058;
    let t19060 = t3482 * t19059;
    let t19062 = t2266 * t3575;
    let t19063 = t3484 * t19062;
    let t19064 = t5633 * t19063;
    let t19067 = t140 * t1336 * t454;
    let t19068 = t2153 * t3742;
    let t19069 = t3796 * t19068;
    let t19070 = t19067 * t19069;
    let t19072 = t13311 * t6229;
    (t19055, t19056, t19058, t19060, t19062, t19064, t19067, t19068, t19070, t19072)
}
