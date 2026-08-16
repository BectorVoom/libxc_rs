//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 955/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk955(t2508: f64, t2580: f64, t28023: f64, t2958: f64, t3009: f64, t7226: f64, t43288: f64, t43289: f64, t43290: f64, t43295: f64, t43298: f64, t43300: f64, t43302: f64, t43304: f64, t43307: f64, t43312: f64, t43315: f64, t43318: f64, t43321: f64, t43325: f64, t43326: f64, t43330: f64) -> f64 {
    let t43335 = 0.92286314761706691403e-1_f64 * t2508 * t2580 * t2958 * t28023;
    let t43339 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t3009 * t28023;
    let t43340 = t43288 - t43289 - 0.85450291446024714264e-3_f64 * t43290 + t43295 - 0.92286314761706691402e-1_f64 * t43298 + t43300 - 0.10766736722199113997e0_f64 * t43302 + 0.20508069947045931423e-1_f64 * t43304 + 0.15381052460284448567e-1_f64 * t2508 * t2580 * t43307 + t43312 + t43315 + 0.30762104920568897134e-1_f64 * t43318 + t43321 + t43325 - 0.64087718584518535698e-3_f64 * t43326 - 0.64087718584518535698e-3_f64 * t43330 + t43335 - t43339;
    t43340
}
