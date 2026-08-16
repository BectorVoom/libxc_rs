//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 690/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk690(t3477: f64, t969: f64, t1410: f64, t2473: f64, t967: f64, t2478: f64, t1399: f64, t2484: f64, t952: f64, t2457: f64, t2488: f64, t3461: f64, t3472: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3479 = 1.0_f64 * t3477 * t969;
    let t3481 = 1.0_f64 * t2473 * t1410;
    let t3482 = t1410 * t967;
    let t3484 = 2.0_f64 * t2478 * t3482;
    let t3485 = t2484 * t1399;
    let t3486 = t3485 * t952;
    let t3490 = t2488 - t2457 / 3.0_f64 - t3461 / 3.0_f64 + t3472;
    (t3479, t3481, t3482, t3484, t3485, t3486, t3490)
}
