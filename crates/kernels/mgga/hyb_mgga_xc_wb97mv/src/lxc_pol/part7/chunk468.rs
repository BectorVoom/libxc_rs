//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 468/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk468<F: Float>(t2176: F, t2178: F, t2189: F, t251: F, t782: F, t786: F) -> (F, F, F) {
    let t2191 = t2176 - 0.35616666666666666666e-1 * t2178 + 0.53425e-1 * t2189;
    let t2193 = 0.621814e-1 * t2191 * t251;
    let t2194 = t782 * t786;
    (t2191, t2193, t2194)
}
