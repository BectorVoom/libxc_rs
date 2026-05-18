//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 392/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk392<F: Float>(t1467: F, t418: F, t423: F, t412: F, t413: F, t312: F, t1295: F, t196: F, t400: F, t420: F, t421: F, rho1: F, sigma2: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1468 = F::new(1.0) / t1467;
    let t1469 = t1468 * tau1;
    let t1473 = t418 * rho1;
    let t1474 = F::new(1.0) / t1473;
    let t1475 = t1474 * t423;
    let t1478 = t412 * sigma2;
    let t1479 = t413 * t1478;
    let t1480 = t312 * t1479;
    let t1481 = t418 * t1295;
    let t1483 = F::new(1.0) / t196 / t1481;
    let t1484 = t420 * t400;
    let t1486 = F::new(1.0) / t421 / t1484;
    (t1468, t1469, t1474, t1475, t1478, t1479, t1480, t1483, t1484, t1486)
}
