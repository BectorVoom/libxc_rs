//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 686/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk686<F: Float>(t2211: F, t786: F, t2492: F, t923: F, t1971: F, t327: F, t2598: F, t875: F, t1: F, t350: F, t818: F, t3787: F) -> (F, F, F, F, F) {
    let t7453 = t2211 * t786;
    let t7460 = t2492 * t923;
    let t7502 = t1971 * t327;
    let t7503 = t2598 * t875;
    let t7510 = t818 * t1 * t350;
    let t7511 = t3787 * t7510;
    (t7453, t7460, t7502, t7503, t7511)
}
