//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 523/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk523(t2484: f64, t2485: f64, t2454: f64, t2457: f64, t2468: f64) -> (f64, f64, f64) {
    let t2486 = t2484 * t2485;
    let t2488 = 4.0_f64 / 9.0_f64 * t2454;
    let t2490 = t2488 - 2.0_f64 / 3.0_f64 * t2457 + t2468;
    (t2486, t2488, t2490)
}
