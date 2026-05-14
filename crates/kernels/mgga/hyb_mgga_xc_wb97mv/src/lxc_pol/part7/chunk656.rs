//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 656/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk656<F: Float>(t181: F, t739: F, t178: F, t1264: F, t173: F, t180: F, t3244: F, t3249: F, t3262: F, t3263: F, t3269: F, t746: F, t750: F, t1276: F, t172: F, t184: F, t2127: F, t3248: F, t3252: F, t741: F, t755: F) -> (F, F, F, F) {
    let t3272 = t739 * t181;
    let t3275 = t178 * t739;
    let t3281 = -2.0 * t3262 * t3263 + t746 * t3244 * t180 / 2.0 + t3269 * t3263 / 4.0 - 4.0 * t3272 * t1264 - t3275 * t3249 - 4.0 * t750 * t3244 - t173 * t3244 * t180;
    let t3284 = -t3248 * t3249 / 2.0 + 2.0 * t2127 * t3252 - t741 * t3244 + 2.0 * t3244 * t184 + 2.0 * t1264 * t755 + 2.0 * t739 * t1276 + 2.0 * t172 * t3281;
    (t3272, t3275, t3281, t3284)
}
