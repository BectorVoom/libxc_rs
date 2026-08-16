//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 960/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk960<F: Float>(t1083: F, t5631: F, t1473: F, t2936: F, t1503: F, t8496: F, t1371: F, t1480: F, t8308: F, t413: F, t991: F, t159: F, t285: F) -> (F, F, F, F, F, F) {
    let t26118 = t5631 * t1083;
    let t26129 = t1473 * t2936;
    let t26131 = t1503 * t8496;
    let t26135 = t8308 * t1371 * t1480;
    let t26143 = t413 * t991;
    let t26145 = t26143 * t159 * t285;
    (t26118, t26129, t26131, t26135, t26143, t26145)
}
