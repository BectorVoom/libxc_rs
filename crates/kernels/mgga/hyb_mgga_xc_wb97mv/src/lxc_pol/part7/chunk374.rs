//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 374/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk374<F: Float>(t1357: F, t284: F, t1325: F, t1331: F, t1334: F, t1338: F, t841: F, t844: F) -> (F, F) {
    let t1358 = t1357 * t284;
    let t1364 = 0.258925e1 * t1331 - t841 + 0.905775e0 * t1325 + 0.16504875e0 * t1334 - t844 + 0.248355e0 * t1338;
    (t1358, t1364)
}
