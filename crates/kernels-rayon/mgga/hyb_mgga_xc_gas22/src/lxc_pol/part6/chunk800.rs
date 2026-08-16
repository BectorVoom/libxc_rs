//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 800/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk800(t4436: f64, t4437: f64, t1491: f64, t1502: f64, t409: f64, t429: f64, t428: f64, t1483: f64, t414: f64, t1497: f64, t1297: f64, t1464: f64, t1469: f64, t1492: f64, t1503: f64, t1620: f64, t3978: f64, t3981: f64, t405: f64, t408: f64, t4419: f64, t4428: f64, t4432: f64) -> f64 {
    let t4438 = t4436 * t4437;
    let t4441 = t1491 * t1502;
    let t4447 = 1.0_f64 / t429 / t409;
    let t4448 = t428 * t4447;
    let t4451 = t414 * t1483;
    let t4452 = t4451 * t1497;
    let t4457 = -40.0_f64 / 9.0_f64 * t405 * t3981 - 80.0_f64 / 9.0_f64 * t1464 * t3981 - 80.0_f64 / 9.0_f64 * t408 * t1469 * t1297 + 50.0_f64 / 9.0_f64 * t405 * t3978 + 200.0_f64 / 9.0_f64 * t1464 * t3978 + 50.0_f64 / 3.0_f64 * t408 * t4419 * t1620 + 0.10670320988213624232e1_f64 * t1503 * t4428 + 0.3553815109799485967e0_f64 * t4432 * t4438 + 0.94768402927986292454e0_f64 * t4441 * t4438 + 0.10670320988213624232e1_f64 * t1492 * t4428 + 0.59230251829991432783e0_f64 * t4448 * t4438 - 0.24639784761276436038e1_f64 * t1503 * t4452 - 0.24639784761276436038e1_f64 * t1492 * t4452;
    t4457
}
