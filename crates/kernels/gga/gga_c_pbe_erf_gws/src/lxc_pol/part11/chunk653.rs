//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 653/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk653<F: Float>(t1114: F, t6644: F, t1109: F, t931: F, t6566: F, t1136: F, t6228: F, t2106: F, t1140: F, t6480: F, t1125: F, t6616: F, t19: F, t329: F, t332: F, t838: F, t857: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9035 = t1114 * t6644;
    let t9056 = t1109 * t931;
    let t9119 = t1114 * t6566;
    let t9144 = t6228 * t1136;
    let t9150 = t1109 * t2106;
    let t9176 = t6480 * t1140;
    let t9182 = t1125 * t6616;
    let t9239 = t931 * t19;
    let t9241 = t329 * t332 * t9239;
    let t9246 = t838 * t857;
    (t9035, t9056, t9119, t9144, t9150, t9176, t9182, t9239, t9241, t9246)
}
