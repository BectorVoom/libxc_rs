//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 847/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk847<F: Float>(t2051: F, t6854: F, t2029: F, t137: F, t142: F, t510: F, t5623: F, t1354: F, t1477: F, t1480: F, t551: F, t1371: F, t6037: F) -> (F, F, F, F) {
    let t16386 = t2051 * t6854;
    let t16392 = t2029 * t2029;
    let t16393 = F::cast_from(1.0_f64) / t16392;
    let t16394 = t16393 * t137;
    let t16397 = t16394 * t142 * t5623 * t510;
    let t16404 = t1477 * t1354 * t551 * t1480;
    let t16407 = t6037 * t1371 * t1480;
    (t16386, t16397, t16404, t16407)
}
