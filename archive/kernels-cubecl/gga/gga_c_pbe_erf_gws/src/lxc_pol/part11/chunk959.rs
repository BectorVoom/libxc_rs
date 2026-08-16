//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 959/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk959<F: Float>(t366: F, t991: F, t169: F, t242: F, t1076: F, t413: F, t1383: F, t2994: F, t1378: F, t6056: F, t922: F, t281: F, t285: F, t4576: F) -> (F, F, F, F, F, F) {
    let t26036 = t366 * t991;
    let t26038 = t169 * t26036 * t242;
    let t26051 = t413 * t1076;
    let t26061 = t169 * t2994 * t1383;
    let t26085 = t922 * t991 * t1378 * t6056;
    let t26101 = t281 * t991 * t4576 * t285;
    (t26036, t26038, t26051, t26061, t26085, t26101)
}
