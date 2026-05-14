//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1239/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1239<F: Float>(t10633: F, t10636: F, t10641: F, t10644: F, t10649: F, t10652: F, t10657: F, t10660: F, t10682: F, t10685: F, t10690: F, t10693: F, t2081: F, t3182: F, t3208: F, t3213: F, t3218: F, t3223: F, t3228: F, t8684: F) -> (F,) {
    let t30279 = -t10682 * t2081 / 0.74317824e10 - 2.0 / 3.0 * t10685 * t2081 + t3182 * t8684 / 3.0 + t10690 * t2081 / 6.0 + t10693 * t2081 / 8.0 - t3208 * t8684 / 24.0 - t10633 * t2081 / 48.0 - t10636 * t2081 / 80.0 + t3213 * t8684 / 320.0 + t10641 * t2081 / 640.0 + t10644 * t2081 / 1152.0 - t3218 * t8684 / 5760.0 - t10649 * t2081 / 11520.0 - t10652 * t2081 / 21504.0 + t3223 * t8684 / 129024.0 + t10657 * t2081 / 258048.0 + t10660 * t2081 / 491520.0 - t3228 * t8684 / 3440640.0;
    (t30279,)
}
