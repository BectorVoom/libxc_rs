//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 660/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk660<F: Float>(t222: F, t3326: F, t37: F, t2176: F, t2178: F, t3317: F, t251: F, t1327: F, t786: F) -> (F, F, F, F) {
    let t3328 = t222 * t37 * t3326;
    let t3330 = t2176 - 0.17808333333333333333e-1 * t2178 - 0.17808333333333333333e-1 * t3317 + 0.53425e-1 * t3328;
    let t3332 = 0.621814e-1 * t3330 * t251;
    let t3333 = t1327 * t786;
    (t3328, t3330, t3332, t3333)
}
