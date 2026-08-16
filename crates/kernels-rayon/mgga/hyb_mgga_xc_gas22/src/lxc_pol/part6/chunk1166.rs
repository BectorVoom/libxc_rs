//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1166/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1166(t20685: f64, t241: f64, t238: f64, t243: f64, t6611: f64, t805: f64, t2213: f64, t2220: f64, t2224: f64, t222: f64, t6007: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20686 = t20685 * t241;
    let t20688 = t238 * t20686 * t243;
    let t20689 = 0.13490888888888888889e1_f64 * t20688;
    let t20691 = t238 * t6611 * t805;
    let t20694 = t238 * t2213 * t2220;
    let t20697 = t238 * t2213 * t2224;
    let t20703 = t222 * t6007 * t779;
    (t20686, t20688, t20689, t20691, t20694, t20697, t20703)
}
