//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1275/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1275<F: Float>(t2239: F, t2245: F, t4163: F, t2200: F, t4192: F, t6859: F, t10927: F, t6937: F, t1341: F, t2199: F, t9111: F, t31210: F, t31212: F, t31215: F, t31217: F, t31220: F, t31224: F, t31226: F, t31228: F, t31230: F, t31232: F, t31234: F, t31236: F, t31238: F, t31240: F, t31242: F) -> (F, F, F, F, F) {
    let t31245 = 6.0 * t2245 * t4163 * t2239;
    let t31248 = 0.57895126195293126241e3 * t6859 * t4192 * t2200;
    let t31250 = 8.0 * t6937 * t10927;
    let t31253 = 4.0 * t2199 * t1341 * t9111;
    let t31254 = -t31210 - t31212 - t31215 - t31217 - t31220 - t31224 + t31226 + t31228 + t31230 + t31232 - t31234 - t31236 - t31238 - t31240 - t31242 - t31245 - t31248 + t31250 + t31253;
    (t31245, t31248, t31250, t31253, t31254)
}
