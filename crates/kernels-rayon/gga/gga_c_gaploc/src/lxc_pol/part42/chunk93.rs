//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 93/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk93(t110: f64, t9: f64, t19: f64, t3: f64, t108: f64, t14: f64, t23: f64, t397: f64, t72: f64, t109: f64, t111: f64, t112: f64, t400: f64, t75: f64) -> (f64, f64, f64, f64) {
    let t427 = t9 * t110;
    let t432 = t19 / t3;
    let t433 = t108 * t108;
    let t434 = t433 * t433;
    let t435 = t434 * t108;
    let t436 = t432 * t435;
    let t437 = t23 * t14;
    let t441 = t72 * t397;
    let t447 = 0.13140859333333333333e-2_f64 * t109 * t427 * t112 - 0.98556444999999999995e-3_f64 * t436 * t437 * t112 - 0.19711288999999999999e-2_f64 * t109 * t111 * t441 - 4.0_f64 * t75 * t400;
    (t427, t436, t437, t447)
}
