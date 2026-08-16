//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 767/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk767(t2234: f64, t4143: f64, t2240: f64, t3300: f64, t4106: f64, t1358: f64, t829: f64) -> (f64, f64, f64, f64) {
    let t4145 = 0.16081979498692535067e2_f64 * t2234 * t4143;
    let t4148 = t2240 - 0.34246666666666666666e-1_f64 * t3300 + 0.5137e-1_f64 * t4106;
    let t4153 = t1358 * t1358;
    let t4154 = t4153 * t829;
    (t4145, t4148, t4153, t4154)
}
