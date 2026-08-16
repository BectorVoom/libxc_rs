//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1185/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1185(t2643: f64, t7554: f64, t222: f64, t468: f64, t7327: f64, t7389: f64, t7324: f64, t7374: f64, t1884: f64, t2728: f64, t2732: f64, t567: f64, t7448: f64, t7450: f64) -> (f64, f64, f64, f64, f64) {
    let t21957 = t2643 * t7554;
    let t21959 = t468 * t222;
    let t21962 = 0.1301229756036208781e0_f64 * t21959 * t7389 * t7327;
    let t21965 = 0.19263893255070628431e1_f64 * t21959 * t7374 * t7324;
    let t21969 = 0.22911460125803964958e1_f64 * t222 * t1884 * t2728 * t2732;
    let t21973 = 0.68734380377411894876e1_f64 * t222 * t567 * t7448 * t7450;
    (t21957, t21962, t21965, t21969, t21973)
}
