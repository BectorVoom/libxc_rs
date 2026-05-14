//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 785/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk785<F: Float>(t428: F, t4447: F, t1483: F, t414: F, t1497: F, t1297: F, t1464: F, t1469: F, t1492: F, t1503: F, t1620: F, t3978: F, t3981: F, t405: F, t408: F, t4419: F, t4428: F, t4432: F, t4438: F, t4441: F) -> (F,) {
    let t4448 = t428 * t4447;
    let t4451 = t414 * t1483;
    let t4452 = t4451 * t1497;
    let t4457 = -40.0 / 9.0 * t405 * t3981 - 80.0 / 9.0 * t1464 * t3981 - 80.0 / 9.0 * t408 * t1469 * t1297 + 50.0 / 9.0 * t405 * t3978 + 200.0 / 9.0 * t1464 * t3978 + 50.0 / 3.0 * t408 * t4419 * t1620 + 0.10670320988213624232e1 * t1503 * t4428 + 0.3553815109799485967e0 * t4432 * t4438 + 0.94768402927986292454e0 * t4441 * t4438 + 0.10670320988213624232e1 * t1492 * t4428 + 0.59230251829991432783e0 * t4448 * t4438 - 0.24639784761276436038e1 * t1503 * t4452 - 0.24639784761276436038e1 * t1492 * t4452;
    (t4457,)
}
