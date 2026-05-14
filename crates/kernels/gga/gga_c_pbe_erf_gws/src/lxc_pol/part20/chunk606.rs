//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 606/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk606<F: Float>(t2843: F, t2845: F, t3360: F, t85: F, t2516: F, t1267: F, t1288: F, t1296: F, t1446: F, t1450: F, t3362: F, t3365: F, t3366: F, t3364: F) -> (F, F, F, F, F) {
    let t3367 = 8.0 * t2843;
    let t3368 = 8.0 * t2845;
    let t3369 = t3360 * t85;
    let t3370 = 0.19751789702565206229e-1 * t3369;
    let t3371 = 2.0 * t2516;
    let t3372 = -t1267 + t1288 + t1296 + t1446 + t1450 - t3365 - t3366 - t3367 - t3368 + t3370 + t3371 + t3362;
    let t3373 = t3364 + t3372;
    (t3367, t3368, t3370, t3371, t3373)
}
