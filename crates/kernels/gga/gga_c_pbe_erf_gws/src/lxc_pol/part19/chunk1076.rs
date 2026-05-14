//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1076/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1076<F: Float>(t2331: F, t864: F, t899: F, t935: F, t1477: F, t360: F, t56: F, t863: F, t4029: F, t14083: F, t888: F, t1189: F, t6590: F, t2276: F, t2299: F, t3969: F) -> (F, F, F, F, F, F, F) {
    let t51395 = t899 * t864 * t2331;
    let t51396 = t51395 * t935;
    let t51407 = t863 * t360 * t1477 * t56;
    let t51408 = t51407 * t4029;
    let t51412 = t14083 * t888;
    let t51414 = t1189 * t6590;
    let t51421 = t2276 * t3969 * t2299;
    (t51395, t51396, t51407, t51408, t51412, t51414, t51421)
}
