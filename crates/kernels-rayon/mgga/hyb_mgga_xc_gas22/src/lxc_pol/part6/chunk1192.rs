//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1192/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1192(t1037: f64, t1048: f64, t2711: f64, t2723: f64, t1101: f64, t7536: f64, t1068: f64, t1110: f64, t21846: f64, t2639: f64, t7237: f64, t2643: f64, t7249: f64) -> (f64, f64, f64, f64, f64) {
    let t22076 = 0.4274e0_f64 * t1037 * t2711 * t2723 * t1048;
    let t22080 = t7536 * t1101;
    let t22084 = t7536 * t1068;
    let t22089 = 0.6233709278045326953e3_f64 * t1110 * t7237 * t21846 * t2639;
    let t22090 = t2643 * t7249;
    (t22076, t22080, t22084, t22089, t22090)
}
