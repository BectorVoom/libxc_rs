//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 799/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk799(t3972: f64, t430: f64, t1302: f64, t418: f64, t195: f64, t1479: f64, t1486: f64, t328: f64, t407: f64, t1618: f64, t414: f64, t423: f64, tau1: f64) -> (f64, f64, f64, f64, f64) {
    let t4419 = t430 * t3972;
    let t4423 = t418 * t1302;
    let t4425 = 1.0_f64 / t195 / t4423;
    let t4426 = t1479 * t4425;
    let t4427 = t1486 * tau1;
    let t4428 = t4426 * t4427;
    let t4431 = t328 * t407;
    let t4432 = t4431 * t430;
    let t4433 = t418 * t1618;
    let t4435 = 1.0_f64 / t195 / t4433;
    let t4436 = t414 * t4435;
    let t4437 = t423 * t3972;
    (t4419, t4428, t4432, t4436, t4437)
}
