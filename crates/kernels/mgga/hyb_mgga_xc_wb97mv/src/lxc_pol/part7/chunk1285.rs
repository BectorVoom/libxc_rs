//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1285/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1285<F: Float>(t31140: F, t31143: F, t31146: F, t31149: F, t31151: F, t31156: F, t31159: F, t31162: F, t31165: F, t31167: F, t31210: F, t31212: F, t31215: F, t31217: F, t31220: F, t31224: F, t31226: F, t31228: F, t31230: F, t31232: F, t31234: F, t31236: F, t31238: F, t31240: F, t31242: F) -> (F, F) {
    let t31497 = -t31140 - t31143 - t31146 - t31149 + t31151 + t31156 + t31159 + t31162 - t31165 + t31167 + t31210 + t31212;
    let t31498 = t31215 + t31217 + t31220 + t31224 - t31226 - t31228 - t31230 - t31232 + t31234 + t31236 + t31238 + t31240 + t31242;
    (t31497, t31498)
}
