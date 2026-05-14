//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 546/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk546<F: Float>(t2450: F, t2498: F, t2453: F, t2464: F, t2482: F, t2487: F, t2493: F, t2495: F, t2501: F, t2505: F, t2509: F) -> (F, F, F) {
    let t2580 = 0.40256666666666666667e0 * t2450;
    let t2585 = 0.137975e0 * t2498;
    let t2589 = -0.1294625e1 * t2482 + 0.258925e1 * t2487 + t2580 - 0.60385e0 * t2453 + 0.905775e0 * t2464 + 0.82524375e-1 * t2493 + 0.16504875e0 * t2495 + t2585 - 0.33114e0 * t2501 + 0.248355e0 * t2505 + 0.248355e0 * t2509;
    (t2580, t2585, t2589)
}
