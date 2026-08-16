//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 392/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk392(t1467: f64, t418: f64, t423: f64, t412: f64, t413: f64, t312: f64, t1295: f64, t196: f64, t400: f64, t420: f64, t421: f64, rho1: f64, sigma2: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1468 = 1.0_f64 / t1467;
    let t1469 = t1468 * tau1;
    let t1473 = t418 * rho1;
    let t1474 = 1.0_f64 / t1473;
    let t1475 = t1474 * t423;
    let t1478 = t412 * sigma2;
    let t1479 = t413 * t1478;
    let t1480 = t312 * t1479;
    let t1481 = t418 * t1295;
    let t1483 = 1.0_f64 / t196 / t1481;
    let t1484 = t420 * t400;
    let t1486 = 1.0_f64 / t421 / t1484;
    (t1468, t1469, t1474, t1475, t1478, t1479, t1480, t1483, t1484, t1486)
}
