//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 595/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk595<F: Float>(t2300: F, t3189: F, t904: F, t3166: F, t916: F, t1123: F, t2313: F, t2255: F, t2279: F, t3258: F, t3038: F, t824: F, t905: F, t1113: F, t2271: F, t1154: F, t2289: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3279 = t2300 * t904 * t3189;
    let t3282 = t904 * t3166;
    let t3283 = t916 * t3282;
    let t3286 = t1123 * t2313;
    let t3287 = t2255 * t3286;
    let t3290 = t3258 * t2279;
    let t3291 = t2255 * t3290;
    let t3294 = t3038 * t824;
    let t3295 = t905 * t3294;
    let t3298 = t1113 * t2271;
    let t3299 = t905 * t3298;
    let t3302 = t2289 * t1154;
    (t3279, t3283, t3287, t3291, t3294, t3295, t3298, t3299, t3302)
}
