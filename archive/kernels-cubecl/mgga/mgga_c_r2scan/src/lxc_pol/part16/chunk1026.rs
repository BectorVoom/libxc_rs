//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1026/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1026<F: Float>(t312: F, t320: F, t6659: F, t325: F, t326: F, t6691: F, t1337: F, t1347: F, t260: F, t277: F, t481: F, t1541: F, t57: F) -> (F, F, F, F, F, F, F) {
    let t19155 = t312 / t6659 / t320;
    let t19203 = t325 / t6691 / t326;
    let t19309 = F::cast_from(1.0_f64) / t1347 / t1337;
    let t19326 = t1347 * t1347;
    let t19327 = F::cast_from(1.0_f64) / t19326;
    let t19790 = t260 * t277;
    let t19791 = t19790 * t481;
    let t19839 = t57 * t1541;
    (t19155, t19203, t19309, t19327, t19790, t19791, t19839)
}
