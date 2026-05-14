//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1216/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1216<F: Float>(t43: F, t29282: F, t29320: F, t29363: F, t29408: F, t1842: F, t3901: F, t3903: F, t3905: F, t3907: F, t3909: F, t3911: F, t3913: F, t3915: F, t3917: F, t3919: F, t3921: F, t3923: F, t587: F, t591: F, t595: F, t599: F, t603: F, t607: F) -> (F, F, F) {
    let t45 = 0.135e1 < t43;
    let t29410 = t29282 + t29320 + t29363 + t29408;
    let t29411 = piecewise3(t45, t29410, 0.0);
    let t29424 = -7.0 / 8.0 * t3901 * t1842 + t3903 * t1842 / 8.0 + 9.0 / 80.0 * t3905 * t1842 - t3907 * t1842 / 80.0 - 11.0 / 1152.0 * t3909 * t1842 + t3911 * t1842 / 1152.0 + 13.0 / 21504.0 * t3913 * t1842 - t3915 * t1842 / 21504.0 - t3917 * t1842 / 32768.0 + t3919 * t1842 / 491520.0 + 17.0 / 13271040.0 * t3921 * t1842 - t3923 * t1842 / 13271040.0 + t587 * t29411 / 240.0 - t591 * t29411 / 4480.0 + t595 * t29411 / 103680.0 - t599 * t29411 / 2838528.0 + t603 * t29411 / 89456640.0 - t607 * t29411 / 0.31850496e10;
    (t29410, t29411, t29424)
}
