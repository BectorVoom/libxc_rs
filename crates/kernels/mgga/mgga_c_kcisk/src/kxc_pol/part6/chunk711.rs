//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 711/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk711<F: Float>(t2211: F, t3783: F, t13955: F, t2178: F, t13900: F, t2163: F, t1309: F, t2160: F, t3981: F, t164: F, t2169: F, t2159: F, t3934: F, t394: F, t1224: F, t13524: F, t2075: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t19848 = t2211 * t3783;
    let t19849 = t19848 * sigma0;
    let t19948 = t13955 * t2178;
    let t20127 = t13900 * t2163;
    let t20128 = t1309 * t20127;
    let t20169 = t2160 * t3981;
    let t20184 = t164 * t2169;
    let t20185 = t1309 * t20184;
    let t20255 = t2159 * t394 * t3934;
    let t20292 = t1224 * t13524 * t2075;
    (t19848, t19849, t19948, t20128, t20169, t20185, t20255, t20292)
}
