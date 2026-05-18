//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 445/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk445<F: Float>(t1098: F, t19: F, t796: F, t801: F, t1402: F, t950: F, t1412: F, t954: F, t1523: F, t1528: F, t1143: F, t376: F) -> (F, F, F, F, F, F, F) {
    let t2454 = t1098 * t796 * t19;
    let t2455 = t2454 * t801;
    let t2457 = t1402 * t950;
    let t2465 = t1412 * t954;
    let t2477 = t1523 * t950;
    let t2485 = t1528 * t954;
    let t2501 = t1143 * t376;
    (t2454, t2455, t2457, t2465, t2477, t2485, t2501)
}
