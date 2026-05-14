//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1134/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1134<F: Float>(t21425: F, t222: F, t226: F, t12: F, t5: F, t231: F, t243: F, t228: F, t2282: F, t262: F, t2285: F, t2191: F, t2244: F, t2321: F, t275: F, t2324: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22511 = t222 * t21425 * t226;
    let t22512 = 0.5356037037037037037e1 * t22511;
    let t22513 = t12 * t5;
    let t22518 = 1.0 / t231 / t22513 / t243 / t226 / 96.0;
    let t22530 = f64::powf(t228, -0.25e1);
    let t22540 = 280.0 / 81.0 * t22511;
    let t22554 = t2282 * t2282;
    let t22556 = t262 / t22554;
    let t22557 = t2285 * t2285;
    let t22558 = 1.0 / t22557;
    let t22564 = t2191 * t2244;
    let t22567 = t2321 * t2321;
    let t22568 = 1.0 / t22567;
    let t22569 = t275 * t22568;
    let t22570 = t2324 * t2324;
    (t22511, t22512, t22513, t22518, t22530, t22540, t22556, t22558, t22564, t22568, t22569, t22570)
}
