//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 722/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk722<F: Float>(t10697: F, t3247: F, t9647: F, t13019: F, t2580: F, t2508: F, t13023: F, t1024: F, t3266: F, t2936: F, t3255: F, t12613: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13202 = F::new(0.1922631557535556071e-2) * t13201;
    let t13203 = t2580 * t13019;
    let t13204 = t2508 * t13203;
    let t13206 = t2580 * t13023;
    let t13208 = F::new(0.15381052460284448567e-1) * t2508 * t13206;
    let t13209 = t3266 * t1024;
    let t13211 = F::new(0.76905262301422242837e-2) * t2508 * t13209;
    let t13212 = t2936 * t3255;
    let t13214 = F::new(0.23071578690426672851e-1) * t2508 * t13212;
    let t13215 = F::new(0.64087718584518535698e-3) * t12613;
    (t13200, t13202, t13203, t13204, t13206, t13208, t13209, t13211, t13212, t13214, t13215)
}
