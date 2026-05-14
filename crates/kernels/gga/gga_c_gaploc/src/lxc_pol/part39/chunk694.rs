//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 694/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk694<F: Float>(t13188: F, t2508: F, t3251: F, t9014: F, t10628: F, t5539: F, t9647: F, t12605: F, t12609: F, t10697: F, t3247: F, t13019: F, t2580: F, t13023: F, t1024: F, t3266: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13189 = t2508 * t13188;
    let t13191 = t9014 * t3251;
    let t13193 = 0.92286314761706691403e-1 * t2508 * t13191;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13196 = 0.12817543716903707139e-2 * t13195;
    let t13197 = 0.1922631557535556071e-2 * t12605;
    let t13198 = 0.1281754371690370714e-2 * t12609;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13202 = 0.1922631557535556071e-2 * t13201;
    let t13203 = t2580 * t13019;
    let t13204 = t2508 * t13203;
    let t13206 = t2580 * t13023;
    let t13208 = 0.15381052460284448567e-1 * t2508 * t13206;
    let t13209 = t3266 * t1024;
    (t13189, t13191, t13193, t13194, t13196, t13197, t13198, t13200, t13202, t13203, t13204, t13206, t13208, t13209)
}
