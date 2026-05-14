//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1062/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1062<F: Float>(t2302: F, t6505: F, t19859: F, t6659: F, t858: F, t884: F, t2206: F, t6694: F, t2192: F, t6228: F, t20726: F, t6241: F, t1477: F, t2153: F, t863: F, t2160: F) -> (F, F, F, F, F, F) {
    let t21269 = t6505 * t2302;
    let t21274 = 3.0 / 2.0 * t884 * t6659 * t858 * t19859;
    let t21279 = t2206 * t6694;
    let t21280 = 7.0 / 36.0 * t21279;
    let t21285 = t6228 * t2192;
    let t21286 = 35.0 / 72.0 * t21285;
    let t21287 = t20726 * t6241;
    let t21293 = t863 * t2153 * t1477;
    let t21294 = t21293 * t2160;
    (t21269, t21274, t21280, t21286, t21287, t21294)
}
