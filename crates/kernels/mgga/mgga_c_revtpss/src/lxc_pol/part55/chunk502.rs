//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 502/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk502<F: Float>(t2782: F, t4496: F, t1559: F, t72: F, t686: F, t2798: F, t225: F, t2718: F, t213: F, t2783: F, t1568: F, t233: F, t869: F, t689: F, t874: F, t822: F) -> (F, F, F, F, F, F, F) {
    let t4497 = t2782 * t4496;
    let t4499 = t1559 * t72;
    let t4500 = t4499 * t686;
    let t4501 = t2798 * t4500;
    let t4503 = t225 * t2718;
    let t4504 = t213 * t4503;
    let t4514 = t213 * t2783;
    let t4518 = t233 * t1568;
    let t4519 = t869 * t4518;
    let t4520 = t689 * t4519;
    let t4522 = t1568 * t72;
    let t4524 = t874 * t4522 * t686;
    let t4526 = t822 * t1568;
    (t4497, t4501, t4504, t4514, t4520, t4524, t4526)
}
