//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 509/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk509(t2362: f64, t902: f64, t2350: f64, t312: f64, t2353: f64, t2356: f64, t1828: f64, t1834: f64, t1840: f64, t1846: f64, t1851: f64, t2345: f64, t2372: f64, t2375: f64, t2376: f64, t2380: f64, t2383: f64, t296: f64, t299: f64, t315: f64, t316: f64, t324: f64, t333: f64, t870: f64, t871: f64, t875: f64, t895: f64, t896: f64) -> f64 {
    let t2386 = t2362 * t902;
    let t2389 = t312 * t2350;
    let t2390 = t2353 * t2356;
    let t2396 = 0.28999131295963805491e1_f64 * t333 * t315 * t2345 * t324 + 0.70082276486377300979e0_f64 * t333 * t2350 * t2353 * t2356 - 0.2854310864347144482e1_f64 * t333 * t895 * t2362 * t902 + 0.458714896073149408e1_f64 * t296 * t1828 * t299 - 40.0_f64 / 9.0_f64 * t875 * t1846 + 0.28999131295963805491e1_f64 * t316 * t2372 + 0.14685052460713464727e1_f64 * t2375 * t1840 * t2376 + 50.0_f64 / 9.0_f64 * t2380 * t1851 + 50.0_f64 / 9.0_f64 * t2383 * t1851 - 0.2854310864347144482e1_f64 * t896 * t2386 + 0.70082276486377300979e0_f64 * t2389 * t2390 - 0.52822214337494074078e1_f64 * t870 * t1834 * t871;
    t2396
}
