//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1009/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1009(t10782: f64, t2530: f64, t2508: f64, t2580: f64, t13206: f64, t7129: f64, t42944: f64, t688: f64, t779: f64, t13225: f64, t2549: f64, t2562: f64, t32179: f64, t883: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43316 = t10782 * t2530;
    let t43318 = t2508 * t2580 * t43316;
    let t43321 = 0.15381052460284448567e-1_f64 * t7129 * t13206;
    let t43325 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t42944 * t688;
    let t43326 = t2549 * t13225;
    let t43330 = t943 * t2562 * t883 * t32179;
    (t43316, t43318, t43321, t43325, t43326, t43330)
}
