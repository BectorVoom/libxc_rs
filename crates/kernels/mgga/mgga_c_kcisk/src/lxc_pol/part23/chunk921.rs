//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 921/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk921<F: Float>(t19032: F, t19033: F, t3484: F, t5633: F, t13306: F, t6230: F, t18953: F, t5634: F, t12825: F, t453: F, t1336: F, t140: F, t12829: F, t470: F, t1337: F, t2211: F) -> (F, F, F, F, F, F, F, F) {
    let t19034 = t19032 * t19033;
    let t19035 = t3484 * t19034;
    let t19036 = t5633 * t19035;
    let t19038 = t13306 * t6230;
    let t19040 = t5634 * t18953;
    let t19041 = t3484 * t19040;
    let t19042 = t5633 * t19041;
    let t19044 = t12825 * t453;
    let t19046 = t140 * t1336 * t19044;
    let t19047 = t470 * t12829;
    let t19048 = t19047 * t19033;
    let t19049 = t3484 * t19048;
    let t19050 = t19046 * t19049;
    let t19053 = t1337 * t2211;
    (t19034, t19036, t19038, t19040, t19042, t19048, t19050, t19053)
}
