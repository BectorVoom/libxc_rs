//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1197/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1197(t40380: f64, t40397: f64, t40400: f64, t42172: f64, t42173: f64, t42176: f64, t42180: f64, t42184: f64, t42188: f64, t42190: f64, t42194: f64, t42198: f64) -> f64 {
    let t48073 = 0.51123901271894332903e0_f64 * t40380;
    let t48074 = 0.38342925953920749677e0_f64 * t40397;
    let t48076 = 0.76685851907841499354e0_f64 * t40400;
    let t48078 = t48073 + t42172 + t42173 + t48074 - 0.11502877786176224903e2_f64 * t42176 - t48076 - 0.92023022289409799224e1_f64 * t42180 + t42184 - t42188 + t42190 - t42194 + t42198;
    t48078
}
