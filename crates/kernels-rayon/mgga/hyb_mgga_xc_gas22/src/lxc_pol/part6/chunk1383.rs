//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1383/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1383(t8973: f64, t9104: f64, t2515: f64, t4273: f64, t7075: f64, t11038: f64, t21537: f64, t2479: f64, t2478: f64, t4270: f64, t11031: f64, t2521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30009 = 12.0_f64 * t9104 * t8973;
    let t30012 = 0.96491876992155210402e2_f64 * t7075 * t4273 * t2515;
    let t30015 = 0.62071215503128080361e4_f64 * t21537 * t11038 * t2479;
    let t30018 = 2.0_f64 * t2478 * t4270 * t2515;
    let t30021 = 0.96491876992155210402e2_f64 * t7075 * t11031 * t2479;
    let t30024 = 0.16081979498692535067e2_f64 * t2521 * t11031 * t2515;
    (t30009, t30012, t30015, t30018, t30021, t30024)
}
