//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 726/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk726<F: Float>(t1531: F, t2817: F, t2823: F, t2828: F, t2832: F, t3724: F, t3725: F, t3729: F, t3733: F, t3737: F, t3741: F, t3743: F, t3748: F, t3752: F, t3760: F, t535: F, t536: F) -> (F, F) {
    let t3767 = -50.0 / 9.0 * t3724 * t3725 + 0.6e-2 * t3729 * t1531 + 0.144e-3 * t2828 * t3733 - 0.144e-3 * t2832 * t3737 - 0.128e-3 * t3741 * t3743 + 0.144e-3 * t2828 * t3748 - 0.144e-3 * t2832 * t3752 + 0.48e-4 * t2817 * t3733 - 0.48e-4 * t2823 * t3737 - 0.128e-3 * t3760 * t3743 + 0.48e-4 * t2817 * t3748 - 0.48e-4 * t2823 * t3752;
    let t3771 = t535 * t536;
    (t3767, t3771)
}
