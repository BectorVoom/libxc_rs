//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 501/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk501<F: Float>(t900: F, t1849: F, t303: F, t306: F, t2362: F, t902: F, t2350: F, t312: F, t2353: F, t2356: F, t1828: F, t1834: F, t1840: F, t1846: F, t1851: F, t2345: F, t2372: F, t2375: F, t296: F, t299: F, t315: F, t316: F, t324: F, t333: F, t870: F, t871: F, t875: F, t895: F, t896: F) -> (F,) {
    let t2376 = 1.0 / t900;
    let t2380 = t303 * t1849;
    let t2383 = t306 * t1849;
    let t2386 = t2362 * t902;
    let t2389 = t312 * t2350;
    let t2390 = t2353 * t2356;
    let t2396 = 0.28999131295963805491e1 * t333 * t315 * t2345 * t324 + 0.70082276486377300979e0 * t333 * t2350 * t2353 * t2356 - 0.2854310864347144482e1 * t333 * t895 * t2362 * t902 + 0.458714896073149408e1 * t296 * t1828 * t299 - 40.0 / 9.0 * t875 * t1846 + 0.28999131295963805491e1 * t316 * t2372 + 0.14685052460713464727e1 * t2375 * t1840 * t2376 + 50.0 / 9.0 * t2380 * t1851 + 50.0 / 9.0 * t2383 * t1851 - 0.2854310864347144482e1 * t896 * t2386 + 0.70082276486377300979e0 * t2389 * t2390 - 0.52822214337494074078e1 * t870 * t1834 * t871;
    (t2396,)
}
