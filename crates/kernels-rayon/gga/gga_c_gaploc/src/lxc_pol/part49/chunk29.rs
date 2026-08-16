//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 29/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk29(t110: f64, t22: f64, t70: f64, t75: f64, t109: f64) -> (f64, f64, f64, f64) {
    let t111 = t22 * t110;
    let t112 = 1.0_f64 / t70;
    let t116 = t75 * t75;
    let t118 = 0.19711288999999999999e-2_f64 * t109 * t111 * t112 - 2.0_f64 * t116;
    let t119 = 1.0_f64 / t118;
    (t111, t112, t118, t119)
}
