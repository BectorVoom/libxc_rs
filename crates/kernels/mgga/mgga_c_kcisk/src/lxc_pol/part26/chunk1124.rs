//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1124/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1124<F: Float>(t6229: F, t9461: F, t1339: F, t32045: F, t6225: F, t3924: F, t442: F, t1327: F, t2059: F, t6183: F) -> (F, F, F, F, F, F, F, F) {
    let t33349 = t9461 * t6229;
    let t33350 = t1339 * t33349;
    let t33352 = t32045 * t6225;
    let t33353 = t1339 * t33352;
    let t33357 = t3924 * t442;
    let t33358 = t2059 * t1327;
    let t33359 = t33357 * t33358;
    let t33360 = t6183 * t33359;
    (t33349, t33350, t33352, t33353, t33357, t33358, t33359, t33360)
}
