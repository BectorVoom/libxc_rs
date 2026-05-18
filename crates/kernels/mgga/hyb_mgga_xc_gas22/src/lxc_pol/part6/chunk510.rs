//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 510/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk510<F: Float>(t315: F, t899: F, t913: F, t307: F, t328: F, t332: F, t1570: F, t319: F, t98: F, t1849: F, t324: F, t907: F, t918: F) -> (F, F, F, F) {
    let t2397 = t315 * t899;
    let t2398 = t2397 * t913;
    let t2403 = t328 * t307;
    let t2404 = t2403 * t332;
    let t2405 = t319 * t1570;
    let t2407 = F::new(1.0) / t98 / t2405;
    let t2408 = t315 * t2407;
    let t2409 = t324 * t1849;
    let t2410 = t2408 * t2409;
    let t2413 = t907 * t918;
    (t2398, t2404, t2410, t2413)
}
