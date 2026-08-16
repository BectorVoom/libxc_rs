//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1382/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1382(t3513: f64, t2478: f64, t968: f64, t2477: f64, t4238: f64, t2480: f64, t2521: f64, t2523: f64, t25468: f64, t8990: f64, t2479: f64, t4244: f64, t7075: f64) -> (f64, f64, f64, f64, f64) {
    let t29993 = t3513 * t3513;
    let t29996 = 4.0_f64 * t2478 * t29993 * t968;
    let t29997 = t4238 * t2477;
    let t29999 = 2.0_f64 * t29997 * t2480;
    let t30002 = 0.32163958997385070134e2_f64 * t2521 * t29993 * t2523;
    let t30004 = 0.38596750796862084161e3_f64 * t25468 * t8990;
    let t30007 = 24.0_f64 * t7075 * t4244 * t2479;
    (t29996, t29999, t30002, t30004, t30007)
}
