//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 462/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk462<F: Float>(t2123: F, t2143: F, t173: F, t178: F, t180: F, t181: F, t2115: F, t2135: F, t746: F, t750: F, t172: F, t184: F, t2124: F, t2127: F, t739: F, t741: F, t755: F) -> (F, F, F) {
    let t2144 = t2143 * t2123;
    let t2155 = -2.0 * t2135 * t2123 * t180 + t746 * t2115 * t180 / 2.0 + t2144 * t180 / 4.0 - 4.0 * t2123 * t181 - t178 * t2123 * t180 - 4.0 * t750 * t2115 - t173 * t2115 * t180;
    let t2158 = -t2124 * t180 / 2.0 + 2.0 * t2127 * t2123 - t741 * t2115 + 2.0 * t2115 * t184 + 4.0 * t739 * t755 + 2.0 * t172 * t2155;
    (t2144, t2155, t2158)
}
