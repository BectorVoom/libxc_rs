//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 497/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk497(t2164: f64, t2215: f64, t2167: f64, t2178: f64, t2196: f64, t2201: f64, t2207: f64, t2209: f64, t2218: f64, t2222: f64, t2226: f64) -> (f64, f64, f64) {
    let t2297 = 0.40256666666666666667e0_f64 * t2164;
    let t2302 = 0.137975e0_f64 * t2215;
    let t2306 = -0.1294625e1_f64 * t2196 + 0.258925e1_f64 * t2201 + t2297 - 0.60385e0_f64 * t2167 + 0.905775e0_f64 * t2178 + 0.82524375e-1_f64 * t2207 + 0.16504875e0_f64 * t2209 + t2302 - 0.33114e0_f64 * t2218 + 0.248355e0_f64 * t2222 + 0.248355e0_f64 * t2226;
    (t2297, t2302, t2306)
}
