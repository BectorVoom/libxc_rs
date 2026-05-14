//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 868/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk868<F: Float>(t13191: F, t7129: F, t24660: F, t2508: F, t3251: F, t10782: F, t2530: F, t2580: F, t13206: F, t42944: F, t688: F, t779: F, t13225: F, t2549: F, t2562: F, t32179: F, t883: F, t943: F) -> (F, F, F, F, F, F, F, F) {
    let t43312 = 0.92286314761706691403e-1 * t7129 * t13191;
    let t43315 = 0.92286314761706691403e-1 * t2508 * t24660 * t3251;
    let t43316 = t10782 * t2530;
    let t43318 = t2508 * t2580 * t43316;
    let t43321 = 0.15381052460284448567e-1 * t7129 * t13206;
    let t43325 = 0.76905262301422242837e-2 * t2508 * t779 * t42944 * t688;
    let t43326 = t2549 * t13225;
    let t43330 = t943 * t2562 * t883 * t32179;
    (t43312, t43315, t43316, t43318, t43321, t43325, t43326, t43330)
}
