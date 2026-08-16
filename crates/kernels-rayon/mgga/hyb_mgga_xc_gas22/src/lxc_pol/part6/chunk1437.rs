//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1437/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1437(t4574: f64, t537: f64, t11353: f64, t9507: f64, t11360: f64, t2884: f64, t11348: f64, t11605: f64, t22943: f64, t2821: f64, t2829: f64, t2853: f64, t2859: f64, t30841: f64, t30871: f64, t30892: f64, t30895: f64, t30903: f64, t30975: f64, t3680: f64, t3733: f64, t4494: f64, t7643: f64, t7806: f64, t9632: f64, t9639: f64, t9654: f64, tau1: f64) -> (f64, f64, f64, f64) {
    let t31207 = t4574 * t537;
    let t31222 = t11353 * t9507;
    let t31225 = t11360 * t9507;
    let t31228 = t2884 * tau1;
    let t31229 = t31228 * t11348;
    let t31236 = -64.0_f64 / 81.0_f64 * t31207 * t2853 - 32.0_f64 / 27.0_f64 * t11605 * t2859 - 32.0_f64 / 27.0_f64 * t2821 * t30892 + 32.0_f64 / 27.0_f64 * t2829 * t30895 - 64.0_f64 / 27.0_f64 * t3680 * t30903 - 64.0_f64 / 81.0_f64 * t3733 * t30841 + 32.0_f64 * t7806 * t30871 + 704.0_f64 / 27.0_f64 * t9632 * t31222 - 1408.0_f64 / 81.0_f64 * t9654 * t31225 - 6400.0_f64 / 81.0_f64 * t9639 * t31229 + 32.0_f64 / 9.0_f64 * t22943 * t4494 - 32.0_f64 / 9.0_f64 * t7643 * t30975;
    (t31222, t31225, t31229, t31236)
}
