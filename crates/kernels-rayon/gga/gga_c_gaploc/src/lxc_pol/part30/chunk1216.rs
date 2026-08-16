//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1216/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1216(t2558: f64, t8844: f64, t943: f64, t2508: f64, t25331: f64, t2541: f64, t25335: f64, t7157: f64, t10643: f64, t7137: f64, t7226: f64, t7291: f64, t8483: f64) -> (f64, f64, f64, f64, f64) {
    let t32268 = t943 * t8844 * t2558;
    let t32269 = 0.32043859292259267849e-3_f64 * t32268;
    let t32272 = 0.11535789345213336425e0_f64 * t2508 * t2541 * t25331;
    let t32275 = 0.38452631150711121418e0_f64 * t2508 * t7157 * t25335;
    let t32277 = 0.14355648962932151996e0_f64 * t7137 * t10643;
    let t32281 = 0.92286314761706691402e-1_f64 * t2508 * t7226 * t8483 * t7291;
    (t32269, t32272, t32275, t32277, t32281)
}
