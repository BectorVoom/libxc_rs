//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 991/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk991(t17073: f64, t17075: f64, t17077: f64, t17079: f64, t17081: f64, t17083: f64, t17090: f64, t17094: f64, t17098: f64, t17101: f64, t17103: f64, t1464: f64, t713: f64) -> (f64, f64) {
    let t18213 = t17073 - t17075 + t17077 + t17079 - t17081 - t17083 + t17090 - t17094 + t17098 + t17101 + t17103;
    let t18215 = 0.19208479012345679012e0_f64 * t1464 * t713;
    (t18213, t18215)
}
