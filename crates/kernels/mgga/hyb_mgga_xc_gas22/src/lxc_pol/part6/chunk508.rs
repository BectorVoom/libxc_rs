//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 508/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk508<F: Float>(t2355: F, t317: F, t319: F, t99: F, t2345: F, t324: F, t295: F, t894: F, t900: F, t1849: F, t303: F, t306: F) -> (F, F, F, F, F, F, F) {
    let t2356 = F::cast_from(1.0_f64) / t2355;
    let t2360 = t319 * t317;
    let t2362 = F::cast_from(1.0_f64) / t99 / t2360;
    let t2372 = t2345 * t324;
    let t2375 = t295 * t894;
    let t2376 = F::cast_from(1.0_f64) / t900;
    let t2380 = t303 * t1849;
    let t2383 = t306 * t1849;
    (t2356, t2362, t2372, t2375, t2376, t2380, t2383)
}
