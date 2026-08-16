//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 509/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk509<F: Float>(t2362: F, t902: F, t2350: F, t312: F, t2353: F, t2356: F, t1828: F, t1834: F, t1840: F, t1846: F, t1851: F, t2345: F, t2372: F, t2375: F, t2376: F, t2380: F, t2383: F, t296: F, t299: F, t315: F, t316: F, t324: F, t333: F, t870: F, t871: F, t875: F, t895: F, t896: F) -> F {
    let t2386 = t2362 * t902;
    let t2389 = t312 * t2350;
    let t2390 = t2353 * t2356;
    let t2396 = F::cast_from(0.28999131295963805491e1_f64) * t333 * t315 * t2345 * t324 + F::cast_from(0.70082276486377300979e0_f64) * t333 * t2350 * t2353 * t2356 - F::cast_from(0.2854310864347144482e1_f64) * t333 * t895 * t2362 * t902 + F::cast_from(0.458714896073149408e1_f64) * t296 * t1828 * t299 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t875 * t1846 + F::cast_from(0.28999131295963805491e1_f64) * t316 * t2372 + F::cast_from(0.14685052460713464727e1_f64) * t2375 * t1840 * t2376 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t2380 * t1851 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t2383 * t1851 - F::cast_from(0.2854310864347144482e1_f64) * t896 * t2386 + F::cast_from(0.70082276486377300979e0_f64) * t2389 * t2390 - F::cast_from(0.52822214337494074078e1_f64) * t870 * t1834 * t871;
    t2396
}
