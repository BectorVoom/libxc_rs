//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 491/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk491(t2252: f64, t2275: f64, t2164: f64, t2167: f64, t2178: f64) -> (f64, f64, f64) {
    let t2276 = t2252 * t2275;
    let t2279 = 0.12361111111111111111e-1_f64 * t2164;
    let t2282 = t2279 - 0.18541666666666666667e-1_f64 * t2167 + 0.278125e-1_f64 * t2178;
    (t2276, t2279, t2282)
}
