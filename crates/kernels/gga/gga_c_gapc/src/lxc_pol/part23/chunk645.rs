//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 645/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk645<F: Float>(t2211: F, t786: F, t2492: F, t923: F, t1971: F, t327: F, t2598: F, t875: F, t1: F, t350: F, t818: F, t3787: F, t960: F, t966: F, t311: F, t6194: F) -> (F, F, F, F, F, F, F, F) {
    let t7453 = t2211 * t786;
    let t7460 = t2492 * t923;
    let t7502 = t1971 * t327;
    let t7503 = t2598 * t875;
    let t7510 = t818 * t1 * t350;
    let t7511 = t3787 * t7510;
    let t7519 = t960 * t966;
    let t7520 = t875 * t1;
    let t7521 = t7520 * t350;
    let t7522 = t7519 * t7521;
    let t7547 = t311 * t6194;
    (t7453, t7460, t7502, t7503, t7511, t7521, t7522, t7547)
}
