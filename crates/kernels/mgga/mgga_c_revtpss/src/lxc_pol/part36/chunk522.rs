//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 522/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk522<F: Float>(t231: F, t2783: F, t4494: F, t2782: F, t1559: F, t72: F, t686: F, t2798: F, t225: F, t2718: F, t213: F, t1568: F, t233: F, t869: F, t689: F, t874: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4496 = t2783 * t4494 * t231;
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
    (t4496, t4497, t4499, t4500, t4501, t4503, t4504, t4514, t4518, t4519, t4520, t4522, t4524)
}
