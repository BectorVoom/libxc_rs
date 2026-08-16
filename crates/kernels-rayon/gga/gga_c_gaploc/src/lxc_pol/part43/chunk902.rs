//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 902/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk902(t2508: f64, t42944: f64, t688: f64, t779: f64, t2580: f64, t28023: f64, t2958: f64, t3009: f64, t7226: f64, t23575: f64, t3459: f64, t11135: f64, t7324: f64) -> (f64, f64, f64, f64, f64) {
    let t43325 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t42944 * t688;
    let t43335 = 0.92286314761706691403e-1_f64 * t2508 * t2580 * t2958 * t28023;
    let t43339 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t3009 * t28023;
    let t43346 = 4.0_f64 * t23575 * t3459;
    let t43353 = 4.0_f64 * t7324 * t11135;
    (t43325, t43335, t43339, t43346, t43353)
}
