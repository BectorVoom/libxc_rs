//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 511/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk511<F: Float>(t2390: F, t309: F, t1815: F, t326: F, t2397: F, t878: F, t297: F, t298: F, t296: F, t301: F, t97: F, t307: F, t308: F, t313: F, t316: F, t1567: F, t1812: F, t1817: F, t1821: F, t1824: F, t299: F, t314: F, t318: F, t646: F, t873: F, t882: F, t887: F, t892: F) -> (F,) {
    let t2410 = t2390 * t309;
    let t2419 = t326 * t1815;
    let t2423 = t2397 * t878;
    let t2426 = t298 * t297;
    let t2427 = t296 * t2426;
    let t2428 = t301 * t301;
    let t2429 = t2428 * t97;
    let t2430 = 1.0 / t2429;
    let t2432 = 1.0 / t308 / t307;
    let t2433 = t2430 * t2432;
    let t2436 = t313 * t1815;
    let t2439 = t316 * t1815;
    let t2442 = -80.0 / 9.0 * t887 * t1824 - 80.0 / 9.0 * t318 * t892 * t646 + 0.19911111111111111112e0 * t299 * t2410 - 40.0 / 9.0 * t882 * t1812 + 50.0 / 9.0 * t314 * t1821 + 200.0 / 9.0 * t887 * t1821 + 50.0 / 3.0 * t318 * t2419 * t1567 - 0.85333333333333333333e-1 * t873 * t2423 + 0.91022222222222222219e-2 * t2427 * t2433 + 50.0 / 9.0 * t2436 * t1817 + 50.0 / 9.0 * t2439 * t1817;
    (t2442,)
}
