//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 933/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk933<F: Float>(t325: F, t326: F, t6691: F, t1337: F, t1347: F, t260: F, t277: F) -> (F, F, F, F) {
    let t19203 = t325 / t6691 / t326;
    let t19309 = 1.0 / t1347 / t1337;
    let t19326 = t1347 * t1347;
    let t19327 = 1.0 / t19326;
    let t19790 = t260 * t277;
    (t19203, t19309, t19327, t19790)
}
