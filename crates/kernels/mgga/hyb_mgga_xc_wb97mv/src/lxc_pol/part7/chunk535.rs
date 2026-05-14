//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 535/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk535<F: Float>(t2535: F, t976: F, t2450: F, t2498: F, t2453: F, t2464: F, t2482: F, t2487: F, t2493: F, t2495: F, t2501: F, t2505: F, t2509: F) -> (F, F, F, F) {
    let t2536 = t2535 * t976;
    let t2541 = 0.68863333333333333333e0 * t2450;
    let t2546 = 0.17365833333333333333e0 * t2498;
    let t2550 = -0.17648625e1 * t2482 + 0.3529725e1 * t2487 + t2541 - 0.103295e1 * t2453 + 0.1549425e1 * t2464 + 0.31558125e0 * t2493 + 0.6311625e0 * t2495 + t2546 - 0.41678e0 * t2501 + 0.312585e0 * t2505 + 0.312585e0 * t2509;
    (t2536, t2541, t2546, t2550)
}
