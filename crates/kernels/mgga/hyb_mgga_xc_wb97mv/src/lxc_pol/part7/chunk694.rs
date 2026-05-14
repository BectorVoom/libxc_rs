//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 694/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk694<F: Float>(t1428: F, t994: F, t2453: F, t2501: F, t2580: F, t2585: F, t3478: F, t3489: F, t3503: F, t3508: F, t3514: F, t3516: F, t3520: F, t3524: F, t3528: F) -> (F, F) {
    let t3582 = t1428 * t994;
    let t3596 = -0.1294625e1 * t3503 + 0.258925e1 * t3508 + t2580 - 0.301925e0 * t2453 - 0.301925e0 * t3478 + 0.905775e0 * t3489 + 0.82524375e-1 * t3514 + 0.16504875e0 * t3516 + t2585 - 0.16557e0 * t2501 - 0.16557e0 * t3520 + 0.248355e0 * t3524 + 0.248355e0 * t3528;
    (t3582, t3596)
}
