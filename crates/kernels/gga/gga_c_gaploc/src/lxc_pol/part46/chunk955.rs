//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 955/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk955<F: Float>(t2508: F, t2580: F, t28023: F, t2958: F, t3009: F, t7226: F, t43288: F, t43289: F, t43290: F, t43295: F, t43298: F, t43300: F, t43302: F, t43304: F, t43307: F, t43312: F, t43315: F, t43318: F, t43321: F, t43325: F, t43326: F, t43330: F) -> F {
    let t43335 = F::new(0.92286314761706691403e-1) * t2508 * t2580 * t2958 * t28023;
    let t43339 = F::new(0.46143157380853345701e-1) * t2508 * t7226 * t3009 * t28023;
    let t43340 = t43288 - t43289 - F::new(0.85450291446024714264e-3) * t43290 + t43295 - F::new(0.92286314761706691402e-1) * t43298 + t43300 - F::new(0.10766736722199113997e0) * t43302 + F::new(0.20508069947045931423e-1) * t43304 + F::new(0.15381052460284448567e-1) * t2508 * t2580 * t43307 + t43312 + t43315 + F::new(0.30762104920568897134e-1) * t43318 + t43321 + t43325 - F::new(0.64087718584518535698e-3) * t43326 - F::new(0.64087718584518535698e-3) * t43330 + t43335 - t43339;
    t43340
}
