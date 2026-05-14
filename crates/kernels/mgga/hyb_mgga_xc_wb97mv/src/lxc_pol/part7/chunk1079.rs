//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1079/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1079<F: Float>(t11323: F, t938: F, t4322: F, t7371: F, t11267: F, t11282: F, t7192: F, t7214: F, t9271: F, t9393: F, t385: F, t4287: F, t937: F, t958: F, t1404: F, t9444: F) -> (F, F, F, F, F, F, F) {
    let t11325 = 1.0 * t938 * t11323;
    let t11327 = 0.16081979498692535067e2 * t7371 * t4322;
    let t11332 = -t7214 + 0.12361111111111111111e-1 * t7192 + 0.24722222222222222223e-1 * t9271 - t9393 - 0.92708333333333333333e-2 * t11267 + 0.278125e-1 * t11282;
    let t11333 = t11332 * t385;
    let t11336 = t4287 * t937;
    let t11338 = 1.0 * t11336 * t958;
    let t11340 = 2.0 * t9444 * t1404;
    (t11325, t11327, t11332, t11333, t11336, t11338, t11340)
}
