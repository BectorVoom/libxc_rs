//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 610/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk610<F: Float>(t1000: F, t1017: F, t1827: F, t587: F, t1006: F, t1019: F, t1663: F, t3342: F) -> (F, F, F, F, F) {
    let t3414 = t1000 * t1017;
    let t3415 = t1827 * t3414;
    let t3417 = 8.0 / 45.0 * t587 * t3415;
    let t3419 = 4.0 / 15.0 * t1006 * t1019;
    let t3421 = t1663 * t3342;
    (t3414, t3415, t3417, t3419, t3421)
}
