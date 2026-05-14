//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 510/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk510<F: Float>(t309: F, t2366: F, t319: F, t325: F, t324: F, t328: F, t868: F, t900: F, t300: F, t301: F, t99: F, t1799: F, t98: F, t1806: F, t1824: F, t2357: F, t2360: F, t2363: F, t298: F, t314: F, t327: F, t332: F, t872: F, t878: F, t898: F, t906: F, tau0: F) -> (F, F, F) {
    let t2367 = t309 * tau0;
    let t2368 = t2366 * t2367;
    let t2372 = 1.0 / t325 / t319;
    let t2373 = t324 * t2372;
    let t2382 = t328 * t868;
    let t2383 = t2382 * t900;
    let t2388 = t301 * t300;
    let t2390 = 1.0 / t99 / t2388;
    let t2395 = t301 * t1799;
    let t2397 = 1.0 / t98 / t2395;
    let t2404 = 0.26666666666666666666e0 * t2357 * t2360 + 0.7111111111111111111e0 * t2363 * t2360 + 0.17066666666666666667e0 * t898 * t2368 + 0.44444444444444444444e0 * t2373 * t2360 + 0.17066666666666666667e0 * t906 * t2368 + 0.576e0 * t327 * t328 * t1806 * t332 - 0.99555555555555555556e0 * t906 * t2383 - 0.99555555555555555556e0 * t898 * t2383 - 0.25173333333333333333e0 * t327 * t298 * t2390 * t309 + 0.27306666666666666666e-1 * t327 * t872 * t2397 * t878 - 40.0 / 9.0 * t314 * t1824;
    (t2390, t2397, t2404)
}
