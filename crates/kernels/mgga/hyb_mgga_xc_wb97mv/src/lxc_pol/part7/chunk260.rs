//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 260/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk260<F: Float>(t773: F, t802: F, t780: F, t791: F, t796: F, t806: F) -> (F, F, F) {
    let t822 = 0.516475e0 * t773;
    let t825 = 0.104195e0 * t802;
    let t827 = 0.3529725e1 * t791 - t822 + 0.1549425e1 * t780 + 0.6311625e0 * t796 - t825 + 0.312585e0 * t806;
    (t822, t825, t827)
}
