//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 562/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk562<F: Float>(t3913: F, t470: F, t468: F, t415: F, t1406: F, t1446: F, t1327: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t3914 = sigma0 * t3913;
    let t3915 = t3914 * t470;
    let t3916 = t468 * t3915;
    let t3917 = t415 * t3916;
    let t3919 = t1406 * t1446;
    let t3920 = t415 * t3919;
    let t3922 = t1327 * t1327;
    (t3914, t3915, t3916, t3917, t3919, t3920, t3922)
}
