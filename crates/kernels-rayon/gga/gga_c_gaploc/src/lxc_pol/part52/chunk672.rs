//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 672/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk672(t12290: f64, t12315: f64, t12317: f64, t12321: f64, t135: f64, t139: f64, t145: f64, t459: f64, t463: f64, t3102: f64, t137: f64, t4061: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t12323 = t12290 + t12315 + t12317 + t12321;
    let t12380 = 1.0_f64 / t135;
    let t12381 = t12380 * t139;
    let t12383 = t12381 * t145 * t459;
    let t12385 = pi * t463;
    let t12386 = t3102 * t12385;
    let t12389 = 1.0_f64 / t137 / t4061;
    (t12323, t12380, t12381, t12383, t12385, t12386, t12389)
}
