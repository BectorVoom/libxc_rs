//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1010/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1010<F: Float>(t2121: F, t3074: F, t337: F, t6326: F, t6335: F, t814: F, t6192: F, t6203: F, t6253: F, t6258: F, t2105: F, t810: F, t6336: F, t6605: F, t19803: F, t346: F) -> (F, F, F, F, F, F) {
    let t20366 = 7.0 / 48.0 * t3074 * t6335 * t2121 * t337 * t6326 * t814;
    let t20367 = t6203 * t6192;
    let t20370 = t6253 * t6258 / 8.0;
    let t20371 = t2105 * t810;
    let t20376 = t6336 * t6605;
    let t20377 = 7.0 / 24.0 * t20376;
    let t20378 = t19803 * t346;
    (t20366, t20367, t20370, t20371, t20377, t20378)
}
