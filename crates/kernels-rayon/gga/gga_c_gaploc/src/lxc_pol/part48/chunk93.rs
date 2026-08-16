//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 93/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk93(t397: f64, t72: f64, t109: f64, t111: f64, t112: f64, t400: f64, t427: f64, t436: f64, t437: f64, t75: f64) -> f64 {
    let t441 = t72 * t397;
    let t447 = 0.13140859333333333333e-2_f64 * t109 * t427 * t112 - 0.98556444999999999995e-3_f64 * t436 * t437 * t112 - 0.19711288999999999999e-2_f64 * t109 * t111 * t441 - 4.0_f64 * t75 * t400;
    t447
}
