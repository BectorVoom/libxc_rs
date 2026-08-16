//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 146/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk146(t257: f64, t667: f64, t109: f64, t111: f64, t260: f64, t271: f64, t427: f64, t436: f64, t437: f64, t670: f64) -> f64 {
    let t695 = t257 * t667;
    let t701 = 0.33843946638888888889e-3_f64 * t109 * t427 * t271 - 0.25382959979166666667e-3_f64 * t436 * t437 * t271 - 0.50765919958333333334e-3_f64 * t109 * t111 * t695 - 4.0_f64 * t260 * t670;
    t701
}
