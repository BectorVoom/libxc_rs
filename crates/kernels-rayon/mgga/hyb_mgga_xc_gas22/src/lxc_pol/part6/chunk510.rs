//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 510/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk510(t315: f64, t899: f64, t913: f64, t307: f64, t328: f64, t332: f64, t1570: f64, t319: f64, t98: f64, t1849: f64, t324: f64, t907: f64, t918: f64) -> (f64, f64, f64, f64) {
    let t2397 = t315 * t899;
    let t2398 = t2397 * t913;
    let t2403 = t328 * t307;
    let t2404 = t2403 * t332;
    let t2405 = t319 * t1570;
    let t2407 = 1.0_f64 / t98 / t2405;
    let t2408 = t315 * t2407;
    let t2409 = t324 * t1849;
    let t2410 = t2408 * t2409;
    let t2413 = t907 * t918;
    (t2398, t2404, t2410, t2413)
}
