//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 813/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk813<F: Float>(t2308: F, t2319: F, t1477: F, t855: F, t863: F, t888: F, t838: F, t864: F, t2173: F, t1452: F, t339: F, t2264: F, t899: F, t922: F) -> (F, F, F, F, F, F, F) {
    let t6477 = t2319 * t2308;
    let t6480 = t863 * t855 * t1477;
    let t6481 = t6480 * t888;
    let t6484 = t863 * t864 * t838;
    let t6485 = t6484 * t2173;
    let t6491 = t1452 * t339;
    let t6501 = t899 * t2264 * t922;
    (t6477, t6480, t6481, t6484, t6485, t6491, t6501)
}
