//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1164/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1164(t13924: f64, t7137: f64, t2508: f64, t2580: f64, t47225: f64, t43212: f64, t43216: f64, t43220: f64, t43222: f64, t43224: f64, t43231: f64, t43233: f64, t43237: f64, t43243: f64) -> f64 {
    let t47720 = t7137 * t13924;
    let t47723 = t2508 * t2580 * t47225;
    let t47725 = t43212 + t43216 + t43220 + t43222 + 0.32043859292259267849e-3_f64 * t43224 + 0.76905262301422242837e-2_f64 * t43231 + 0.15381052460284448567e-1_f64 * t43233 - t43237 - 0.30762104920568897135e-1_f64 * t47720 + 0.15381052460284448567e-1_f64 * t47723 - t43243;
    t47725
}
