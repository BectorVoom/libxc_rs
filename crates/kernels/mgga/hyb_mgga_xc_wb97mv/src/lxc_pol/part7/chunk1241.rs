//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1241/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1241<F: Float>(t10712: F, t8225: F, t10701: F, t2013: F, t25821: F, t25823: F, t25825: F, t25827: F, t25829: F, t25842: F, t25844: F, t25860: F, t25862: F, t25864: F, t571: F, t8645: F) -> (F,) {
    let t30352 = t8225 * t10712;
    let t30358 = 142.0 / 243.0 * t25821 + 10.0 / 729.0 * t25823 - 8.0 / 243.0 * t25825 - 2.0 / 81.0 * t25827 + 8.0 / 81.0 * t25829 - 4.0 / 81.0 * t25842 + 2.0 / 27.0 * t25844 + 16.0 / 243.0 * t25860 - 16.0 / 729.0 * t25862 + 2.0 / 243.0 * t25864 - 44.0 / 243.0 * t30352 - 5.0 / 243.0 * t571 * t8645 * t10701 * t2013;
    (t30358,)
}
