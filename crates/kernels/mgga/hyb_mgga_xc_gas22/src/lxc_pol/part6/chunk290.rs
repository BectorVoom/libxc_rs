//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 290/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk290<F: Float>(t935: F, t961: F, t942: F, t953: F, t958: F, t965: F) -> (F, F, F) {
    let t1000 = 0.301925e0 * t935;
    let t1003 = 0.82785e-1 * t961;
    let t1005 = 0.258925e1 * t953 - t1000 + 0.905775e0 * t942 + 0.16504875e0 * t958 - t1003 + 0.248355e0 * t965;
    (t1000, t1003, t1005)
}
