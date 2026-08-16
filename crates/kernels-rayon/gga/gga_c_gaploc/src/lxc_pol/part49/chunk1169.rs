//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1169/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1169(t13934: f64, t2549: f64, t2562: f64, t38974: f64, t883: f64, t943: f64, t43312: f64, t43315: f64, t43318: f64, t43321: f64, t43325: f64, t43326: f64, t43330: f64, t43335: f64, t43339: f64) -> f64 {
    let t47768 = t2549 * t13934;
    let t47772 = t943 * t2562 * t883 * t38974;
    let t47777 = -0.32043859292259267849e-3_f64 * t47768 - 0.32043859292259267849e-3_f64 * t47772 + t43312 + t43315 + 0.15381052460284448567e-1_f64 * t43318 + t43321 + t43325 - 0.32043859292259267849e-3_f64 * t43326 - 0.32043859292259267849e-3_f64 * t43330 + t43335 - t43339;
    t47777
}
