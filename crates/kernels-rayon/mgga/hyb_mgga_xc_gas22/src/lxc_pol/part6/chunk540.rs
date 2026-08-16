//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 540/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk540(t2539: f64, t2562: f64, t2454: f64, t2457: f64, t2468: f64) -> (f64, f64, f64) {
    let t2563 = t2539 * t2562;
    let t2566 = 0.12361111111111111111e-1_f64 * t2454;
    let t2569 = t2566 - 0.18541666666666666667e-1_f64 * t2457 + 0.278125e-1_f64 * t2468;
    (t2563, t2566, t2569)
}
