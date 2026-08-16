//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1440/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1440(t2893: f64, t4576: f64, t1849: f64, t502: f64, t1572: f64, t31247: f64, t513: f64, t1129: f64, t11552: f64, t11520: f64, t11342: f64, t9507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31304 = t4576 * t2893;
    let t31309 = t502 * t1849;
    let t31310 = t31309 * t1572;
    let t31311 = t31247 * t513;
    let t31317 = t11552 * t1129;
    let t31322 = t11520 * t1129;
    let t31330 = t11342 * t9507;
    (t31304, t31310, t31311, t31317, t31322, t31330)
}
