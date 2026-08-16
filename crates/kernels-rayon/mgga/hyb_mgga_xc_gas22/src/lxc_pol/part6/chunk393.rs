//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 393/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk393(t1483: f64, t1486: f64, t209: f64, t328: f64, t407: f64, t430: f64, t194: f64, t418: f64, t196: f64, t414: f64, t423: f64, t211: f64, t429: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1487 = t1483 * t1486;
    let t1491 = t328 * t407 * t209;
    let t1492 = t1491 * t430;
    let t1493 = t418 * t194;
    let t1495 = 1.0_f64 / t196 / t1493;
    let t1496 = t414 * t1495;
    let t1497 = t423 * tau1;
    let t1498 = t1496 * t1497;
    let t1502 = 1.0_f64 / t429 / t211;
    (t1487, t1491, t1492, t1493, t1497, t1498, t1502)
}
