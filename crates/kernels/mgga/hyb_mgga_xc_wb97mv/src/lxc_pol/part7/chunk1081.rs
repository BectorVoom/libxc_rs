//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1081/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1081<F: Float>(t11297: F, t11299: F, t11304: F, t11308: F, t11311: F, t11315: F, t11319: F, t7292: F, t7294: F, t9342: F, t9345: F, t9360: F, t11363: F) -> (F,) {
    let t11373 = 0.82524375e-1 * t11297 + 0.16504875e0 * t11299 - t7292 + 0.27595e0 * t7294 + 0.5519e0 * t9360 - t9342 - t9345 - 0.16557e0 * t11304 + 0.49671e0 * t11308 - 0.16557e0 * t11311 + 0.248355e0 * t11315 + 0.248355e0 * t11319;
    let t11374 = t11363 + t11373;
    (t11374,)
}
