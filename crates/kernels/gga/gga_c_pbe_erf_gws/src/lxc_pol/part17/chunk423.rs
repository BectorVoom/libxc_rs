//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 423/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk423<F: Float>(t128: F, t1533: F, t10: F, t125: F, t390: F, t1251: F, t1: F, t501: F, t506: F, t1515: F, t1243: F, t502: F) -> (F, F, F, F, F, F, F, F) {
    let t1548 = t128 * t1533;
    let t1549 = t10 * t1548;
    let t1552 = t125 * t390;
    let t1553 = t1552 * t128;
    let t1555 = F::new(0.16322666666666666667e0) * t1553 * t1251;
    let t1557 = t501 * t506 * t1;
    let t1558 = t1557 * t1515;
    let t1561 = F::new(0.32645333333333333333e0) * t502 * t1243;
    (t1548, t1549, t1552, t1553, t1555, t1557, t1558, t1561)
}
