//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 508/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk508(t2355: f64, t317: f64, t319: f64, t99: f64, t2345: f64, t324: f64, t295: f64, t894: f64, t900: f64, t1849: f64, t303: f64, t306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2356 = 1.0_f64 / t2355;
    let t2360 = t319 * t317;
    let t2362 = 1.0_f64 / t99 / t2360;
    let t2372 = t2345 * t324;
    let t2375 = t295 * t894;
    let t2376 = 1.0_f64 / t900;
    let t2380 = t303 * t1849;
    let t2383 = t306 * t1849;
    (t2356, t2362, t2372, t2375, t2376, t2380, t2383)
}
