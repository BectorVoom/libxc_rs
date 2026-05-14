//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1041/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1041<F: Float>(t10633: F, t10636: F, t10641: F, t10644: F, t10649: F, t10652: F, t10657: F, t10660: F, t10665: F, t10668: F, t3205: F, t3208: F, t3213: F, t3218: F, t3223: F, t3228: F, t3233: F, t707: F) -> (F,) {
    let t10673 = -t3208 * t3205 / 24.0 - t10633 * t707 / 48.0 - t10636 * t707 / 80.0 + t3213 * t3205 / 320.0 + t10641 * t707 / 640.0 + t10644 * t707 / 1152.0 - t3218 * t3205 / 5760.0 - t10649 * t707 / 11520.0 - t10652 * t707 / 21504.0 + t3223 * t3205 / 129024.0 + t10657 * t707 / 258048.0 + t10660 * t707 / 491520.0 - t3228 * t3205 / 3440640.0 - t10665 * t707 / 6881280.0 - t10668 * t707 / 13271040.0 + t3233 * t3205 / 0.10616832e9;
    (t10673,)
}
