//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1218/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1218<F: Float>(t2558: F, t8844: F, t943: F, t2508: F, t25331: F, t2541: F, t25335: F, t7157: F, t10643: F, t7137: F, t7226: F, t7291: F, t8483: F) -> (F, F, F, F, F) {
    let t32268 = t943 * t8844 * t2558;
    let t32269 = F::cast_from(0.32043859292259267849e-3_f64) * t32268;
    let t32272 = F::cast_from(0.11535789345213336425e0_f64) * t2508 * t2541 * t25331;
    let t32275 = F::cast_from(0.38452631150711121418e0_f64) * t2508 * t7157 * t25335;
    let t32277 = F::cast_from(0.14355648962932151996e0_f64) * t7137 * t10643;
    let t32281 = F::cast_from(0.92286314761706691402e-1_f64) * t2508 * t7226 * t8483 * t7291;
    (t32269, t32272, t32275, t32277, t32281)
}
