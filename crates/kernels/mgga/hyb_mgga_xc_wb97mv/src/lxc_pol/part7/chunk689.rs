//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 689/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk689<F: Float>(t1416: F, t975: F, t2453: F, t2501: F, t2541: F, t2546: F, t3478: F, t3489: F, t3503: F, t3508: F, t3514: F, t3516: F, t3520: F, t3524: F, t3528: F) -> (F, F) {
    let t3549 = t1416 * t975;
    let t3563 = -0.17648625e1 * t3503 + 0.3529725e1 * t3508 + t2541 - 0.516475e0 * t2453 - 0.516475e0 * t3478 + 0.1549425e1 * t3489 + 0.31558125e0 * t3514 + 0.6311625e0 * t3516 + t2546 - 0.20839e0 * t2501 - 0.20839e0 * t3520 + 0.312585e0 * t3524 + 0.312585e0 * t3528;
    (t3549, t3563)
}
