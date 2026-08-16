//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 801/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk801<F: Float>(t191: F, t6382: F, t1: F, t6201: F, t915: F, t2250: F, t2276: F, t814: F, t931: F, t2298: F, t322: F, t2331: F, t899: F, t912: F) -> (F, F, F, F, F, F) {
    let t6383 = t191 * t6382;
    let t6384 = t6383 * t1;
    let t6401 = t6201 * t915;
    let t6402 = t2250 * t6401;
    let t6416 = t2276 * t6401;
    let t6424 = t814 * t931;
    let t6429 = t322 * t2298;
    let t6455 = t899 * t912 * t2331;
    (t6384, t6402, t6416, t6424, t6429, t6455)
}
