//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 991/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk991<F: Float>(t17073: F, t17075: F, t17077: F, t17079: F, t17081: F, t17083: F, t17090: F, t17094: F, t17098: F, t17101: F, t17103: F, t1464: F, t713: F) -> (F, F) {
    let t18213 = t17073 - t17075 + t17077 + t17079 - t17081 - t17083 + t17090 - t17094 + t17098 + t17101 + t17103;
    let t18215 = F::new(0.19208479012345679012e0) * t1464 * t713;
    (t18213, t18215)
}
