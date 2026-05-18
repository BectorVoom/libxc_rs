//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 966/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk966<F: Float>(t1114: F, t20877: F, t1109: F, t2298: F, t21497: F, t1140: F, t21511: F, t1136: F, t21253: F, t21491: F, t3179: F, t21529: F) -> (F, F, F, F, F, F, F) {
    let t27805 = t1114 * t20877;
    let t27917 = t1109 * t2298;
    let t28043 = t1114 * t21497;
    let t28074 = t21511 * t1140;
    let t28173 = t21253 * t1136;
    let t28195 = t21491 * t3179;
    let t28269 = t1114 * t21529;
    (t27805, t27917, t28043, t28074, t28173, t28195, t28269)
}
