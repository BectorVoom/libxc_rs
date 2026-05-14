//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 817/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk817<F: Float>(t528: F, t5420: F, t1917: F, t762: F, t1472: F, t712: F, t713: F, t1464: F, t119: F, t5559: F, t19: F, t5697: F, t799: F, t5385: F, t720: F, t1365: F, t252: F, t254: F) -> (F, F, F, F, F, F, F, F) {
    let t18149 = 0.19947266666666666666e0 * t528 * t5420;
    let t18155 = 0.26596355555555555555e0 * t762 * t1917;
    let t18196 = 0.54024691358024691356e-1 * t712 * t1472 * t713;
    let t18215 = 0.19208479012345679012e0 * t1464 * t713;
    let t18224 = 0.60617527037037037035e-2 * t5559 * t119 * t1917;
    let t18237 = 0.27631489407716049382e-3 * t5697 * t19 * t799 * t713;
    let t18240 = 32.0 / 81.0 * t720 * t5385;
    let t18243 = 56.0 / 243.0 * t252 * t254 * t1365;
    (t18149, t18155, t18196, t18215, t18224, t18237, t18240, t18243)
}
