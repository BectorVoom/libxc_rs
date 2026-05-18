//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 686/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk686<F: Float>(t960: F, t966: F, t1: F, t875: F, t350: F, t311: F, t6194: F, t5: F, t830: F, t2577: F, t869: F, t818: F, t959: F) -> (F, F, F, F, F, F) {
    let t7519 = t960 * t966;
    let t7520 = t875 * t1;
    let t7521 = t7520 * t350;
    let t7522 = t7519 * t7521;
    let t7547 = t311 * t6194;
    let t7549 = t830 * t5;
    let t7553 = t869 * t2577;
    let t7556 = t818 * t959;
    (t7521, t7522, t7547, t7549, t7553, t7556)
}
