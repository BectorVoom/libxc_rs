//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 646/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk646<F: Float>(t13924: F, t2508: F, t12223: F, t2562: F, t883: F, t943: F, t2558: F, t3732: F, t13870: F, t169: F, t299: F, t706: F, t270: F, t13883: F, t738: F, t12277: F, t977: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13925 = t2508 * t13924;
    let t13934 = t2562 * t883 * t12223;
    let t13935 = t943 * t13934;
    let t13937 = t3732 * t2558;
    let t13938 = t943 * t13937;
    let t13941 = t13870 * t169 * t299;
    let t13942 = t706 * t13941;
    let t13944 = 0.76905262301422242837e-2 * t270 * t13942;
    let t13945 = t738 * t13883;
    let t13947 = 0.76905262301422242837e-2 * t270 * t13945;
    let t13951 = t12277 * t977;
    (t13925, t13934, t13935, t13937, t13938, t13941, t13942, t13944, t13945, t13947, t13951)
}
