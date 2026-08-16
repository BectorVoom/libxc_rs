//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 486/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk486(t2252: f64, t829: f64, t2164: f64, t2215: f64, t2167: f64, t2178: f64, t2196: f64, t2201: f64, t2207: f64, t2209: f64, t2218: f64, t2222: f64, t2226: f64) -> (f64, f64, f64, f64) {
    let t2253 = t2252 * t829;
    let t2258 = 0.68863333333333333333e0_f64 * t2164;
    let t2263 = 0.17365833333333333333e0_f64 * t2215;
    let t2267 = -0.17648625e1_f64 * t2196 + 0.3529725e1_f64 * t2201 + t2258 - 0.103295e1_f64 * t2167 + 0.1549425e1_f64 * t2178 + 0.31558125e0_f64 * t2207 + 0.6311625e0_f64 * t2209 + t2263 - 0.41678e0_f64 * t2218 + 0.312585e0_f64 * t2222 + 0.312585e0_f64 * t2226;
    (t2253, t2258, t2263, t2267)
}
