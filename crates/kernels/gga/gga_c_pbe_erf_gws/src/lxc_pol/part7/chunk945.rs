//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 945/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk945<F: Float>(t1763: F, t47: F, t16669: F, t1403: F, t1407: F, t4351: F, t1523: F, t16679: F, t4355: F, t4360: F, t16746: F, t476: F, t1696: F, t52: F, t16973: F, t1413: F, t1416: F, t4366: F) -> (F, F, F, F, F, F, F) {
    let t19058 = 1.0 / t47 / t1763;
    let t19059 = t19058 * t16669;
    let t19062 = t4351 * t1403 * t1407;
    let t19064 = t1523 * t16679;
    let t19066 = t4355 * t4360;
    let t19068 = t476 * t16746;
    let t19071 = 1.0 / t52 / t1696;
    let t19072 = t19071 * t16973;
    let t19075 = t4366 * t1413 * t1416;
    (t19059, t19062, t19064, t19066, t19068, t19072, t19075)
}
