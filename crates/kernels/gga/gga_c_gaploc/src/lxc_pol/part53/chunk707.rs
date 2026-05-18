//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 707/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk707<F: Float>(t13191: F, t2508: F, t10628: F, t5539: F, t9647: F, t10697: F, t3247: F, t13023: F, t2580: F, t1024: F, t3266: F, t2936: F, t3255: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13193 = F::new(0.92286314761706691403e-1) * t2508 * t13191;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13202 = F::new(0.1922631557535556071e-2) * t13201;
    let t13206 = t2580 * t13023;
    let t13208 = F::new(0.15381052460284448567e-1) * t2508 * t13206;
    let t13209 = t3266 * t1024;
    let t13211 = F::new(0.76905262301422242837e-2) * t2508 * t13209;
    let t13212 = t2936 * t3255;
    (t13193, t13194, t13195, t13200, t13202, t13206, t13208, t13209, t13211, t13212)
}
