//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1108/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1108<F: Float>(t1010: F, t31860: F, t1053: F, t9358: F, t3186: F, t3181: F, t10340: F, t9340: F, t2685: F, t3185: F, t3187: F, t113: F, t2932: F, t20: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31861 = t1010 * t31860;
    let t31862 = t9358 * t1053;
    let t31863 = t3186 * t31862;
    let t31864 = 4.0 * t31863;
    let t31865 = t3181 * t9358;
    let t31866 = 2.0 * t31865;
    let t31875 = t10340 * t9340;
    let t31876 = 4.0 * t31875;
    let t31883 = t2685 * t3185;
    let t31884 = t31883 * t3187;
    let t31885 = 2.0 * t31884;
    let t31893 = t2932 * t113;
    let t31894 = t446 * t20;
    (t31861, t31862, t31863, t31864, t31865, t31866, t31875, t31876, t31883, t31884, t31885, t31893, t31894)
}
