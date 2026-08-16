//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 674/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk674(t1359: f64, t828: f64, t2167: f64, t2218: f64, t2258: f64, t2263: f64, t3300: f64, t3311: f64, t3325: f64, t3330: f64, t3336: f64, t3338: f64, t3342: f64, t3346: f64, t3350: f64) -> (f64, f64) {
    let t3371 = t1359 * t828;
    let t3385 = -0.17648625e1_f64 * t3325 + 0.3529725e1_f64 * t3330 + t2258 - 0.516475e0_f64 * t2167 - 0.516475e0_f64 * t3300 + 0.1549425e1_f64 * t3311 + 0.31558125e0_f64 * t3336 + 0.6311625e0_f64 * t3338 + t2263 - 0.20839e0_f64 * t2218 - 0.20839e0_f64 * t3342 + 0.312585e0_f64 * t3346 + 0.312585e0_f64 * t3350;
    (t3371, t3385)
}
