//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 490/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk490<F: Float>(t1158: F, t2323: F, t1150: F, t2319: F, t1154: F, t2289: F, t1120: F, t2246: F, t1146: F, t840: F, t2455: F, t950: F) -> (F, F, F, F, F, F, F) {
    let t3271 = t2323 * t1158;
    let t3274 = t2319 * t1150;
    let t3302 = t2289 * t1154;
    let t3312 = t2246 * t1120;
    let t3321 = t840 * t1146;
    let t3341 = F::cast_from(0.82152657680133333336e0_f64) * t2455;
    let t3342 = t950 * t950;
    (t3271, t3274, t3302, t3312, t3321, t3341, t3342)
}
