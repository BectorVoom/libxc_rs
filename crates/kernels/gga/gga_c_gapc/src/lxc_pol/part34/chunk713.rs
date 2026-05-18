//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 713/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk713<F: Float>(t1: F, t350: F, t786: F, t961: F, t2530: F, t311: F, t6851: F, t442: F, t6856: F, t277: F, t4978: F, t2188: F, t329: F) -> (F, F, F, F, F, F, F) {
    let t7943 = t786 * t1 * t350;
    let t7944 = t961 * t7943;
    let t7949 = t2530 * t350;
    let t7953 = t311 * t6851;
    let t7956 = t6856 * t442;
    let t7967 = t277 * t4978;
    let t7974 = t2188 * t329;
    (t7943, t7944, t7949, t7953, t7956, t7967, t7974)
}
