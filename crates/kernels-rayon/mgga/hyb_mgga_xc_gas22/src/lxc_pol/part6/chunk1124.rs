//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1124/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1124(t4305: f64, t997: f64, t4278: f64, t978: f64, t3579: f64, t3583: f64, t4310: f64, t6996: f64, t1005: f64, t4284: f64, t986: f64, t1422: f64, t3546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11076 = t4305 * t997;
    let t11079 = t4278 * t978;
    let t11086 = t3583 * t3579;
    let t11089 = t4310 * t6996;
    let t11090 = t11089 * t1005;
    let t11095 = t4284 * t986;
    let t11098 = t1422 * t3546;
    (t11076, t11079, t11086, t11089, t11090, t11095, t11098)
}
