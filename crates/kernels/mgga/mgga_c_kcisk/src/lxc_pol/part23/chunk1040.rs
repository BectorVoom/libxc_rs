//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1040/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1040<F: Float>(t20897: F, t1446: F, t5868: F, t415: F, t19033: F, t5634: F, t3484: F, t3482: F, t1220: F, t13448: F, t19710: F, t20019: F, t20886: F, t20892: F, t20893: F, t20896: F, t2174: F, t3925: F, t412: F, t6221: F) -> (F, F, F, F) {
    let t20898 = 0.22109259259259259258e-2 * t20897;
    let t20906 = t5868 * t1446;
    let t20907 = t415 * t20906;
    let t20909 = t5634 * t19033;
    let t20910 = t3484 * t20909;
    let t20911 = t3482 * t20910;
    let t20913 = 0.74498e-1 * t20886 * t3925 + t20892 - 0.44218518518518518517e-2 * t20893 + t20896 - t20898 - 0.193e0 * t13448 * t2174 + 0.193e0 * t1220 * t20019 + 0.193e0 * t6221 * t3925 + t19710 * t412 - 0.13265555555555555555e-1 * t20907 + 0.99491666666666666664e-2 * t20911;
    (t20907, t20909, t20911, t20913)
}
