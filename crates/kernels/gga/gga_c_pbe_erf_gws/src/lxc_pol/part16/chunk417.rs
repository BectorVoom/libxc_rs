//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 417/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk417<F: Float>(t1: F, t501: F, t506: F, t1515: F, t1243: F, t502: F, t505: F, t95: F) -> (F, F, F, F) {
    let t1557 = t501 * t506 * t1;
    let t1558 = t1557 * t1515;
    let t1561 = 0.32645333333333333333e0 * t502 * t1243;
    let t1563 = 1.0 / t505 / t95;
    (t1557, t1558, t1561, t1563)
}
