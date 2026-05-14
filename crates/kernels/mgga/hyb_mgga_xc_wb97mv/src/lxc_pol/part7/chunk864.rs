//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 864/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk864<F: Float>(t1090: F, t2709: F, t1040: F, t2708: F, t492: F, t1873: F, t456: F, t454: F, t6175: F, t1045: F, t453: F, t2685: F, t2697: F, t12: F, t438: F, t444: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7481 = 60.0 * t2709 * t1090;
    let t7482 = t1040 * t2708;
    let t7483 = t7482 * t492;
    let t7486 = 1.0 / t456 / t1873;
    let t7487 = t454 * t7486;
    let t7489 = 120.0 * t7487 * t492;
    let t7491 = 24.0 * t6175 * t492;
    let t7492 = t453 * t1045;
    let t7493 = t7492 * t492;
    let t7495 = t2697 * t2685;
    let t7500 = 1.0 / t438 / t444 * t12 / 4.0;
    (t7481, t7482, t7483, t7487, t7489, t7491, t7492, t7493, t7495, t7500)
}
