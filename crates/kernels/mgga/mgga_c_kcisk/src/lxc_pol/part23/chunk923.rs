//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 923/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk923<F: Float>(t19072: F, t3482: F, t13959: F, t6230: F, t2262: F, t3579: F, t3796: F, t3765: F, t5606: F, t1339: F, t140: F, t5598: F, t5631: F, t1056: F, t220: F) -> (F, F, F, F, F, F, F, F) {
    let t19073 = t3482 * t19072;
    let t19075 = t13959 * t6230;
    let t19076 = 0.14739506172839506172e-2 * t19075;
    let t19077 = t2262 * t3579;
    let t19078 = t3796 * t19077;
    let t19079 = t3482 * t19078;
    let t19081 = t5606 * t3765;
    let t19082 = t1339 * t19081;
    let t19086 = t140 * t5598 * t5631;
    let t19087 = t220 * t1056;
    (t19073, t19075, t19076, t19077, t19079, t19082, t19086, t19087)
}
