//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1087/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1087<F: Float>(t1246: F, t13068: F, t1250: F, t12732: F, t482: F, t1042: F, t1263: F, t3568: F, t1122: F, t247: F, t3372: F, t3634: F) -> (F, F, F, F) {
    let t13069 = t13068 * t1246;
    let t13075 = t482 * t12732 * t1250;
    let t13076 = t1042 * t13075;
    let t13079 = t1263 * t3568;
    let t13080 = t13079 * t1122;
    let t13081 = t1042 * t13080;
    let t13085 = t247 * t3634 * t3372;
    (t13069, t13076, t13081, t13085)
}
