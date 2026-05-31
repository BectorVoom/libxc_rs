//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 93/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk93<F: Float>(t110: F, t9: F, t19: F, t3: F, t108: F, t14: F, t23: F, t397: F, t72: F, t109: F, t111: F, t112: F, t400: F, t75: F) -> (F, F, F, F) {
    let t427 = t9 * t110;
    let t432 = t19 / t3;
    let t433 = t108 * t108;
    let t434 = t433 * t433;
    let t435 = t434 * t108;
    let t436 = t432 * t435;
    let t437 = t23 * t14;
    let t441 = t72 * t397;
    let t447 = F::cast_from(0.13140859333333333333e-2_f64) * t109 * t427 * t112 - F::cast_from(0.98556444999999999995e-3_f64) * t436 * t437 * t112 - F::cast_from(0.19711288999999999999e-2_f64) * t109 * t111 * t441 - F::cast_from(4.0_f64) * t75 * t400;
    (t427, t436, t437, t447)
}
