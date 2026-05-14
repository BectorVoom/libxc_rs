//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1095/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1095<F: Float>(t32268: F, t2508: F, t25331: F, t2541: F, t25335: F, t7157: F, t10643: F, t7137: F, t7226: F, t7291: F, t8483: F, t21446: F, t3009: F, t21783: F, t1850: F, t29273: F, t29280: F, t32253: F, t32256: F, t32259: F, t32261: F, t32266: F, t5396: F) -> (F,) {
    let t32269 = 0.32043859292259267849e-3 * t32268;
    let t32272 = 0.11535789345213336425e0 * t2508 * t2541 * t25331;
    let t32275 = 0.38452631150711121418e0 * t2508 * t7157 * t25335;
    let t32277 = 0.14355648962932151996e0 * t7137 * t10643;
    let t32281 = 0.92286314761706691402e-1 * t2508 * t7226 * t8483 * t7291;
    let t32285 = 0.92286314761706691402e-1 * t2508 * t7226 * t3009 * t21446;
    let t32289 = 0.46143157380853345701e-1 * t2508 * t7226 * t3009 * t21783;
    let t32290 = -t32253 - t32256 + t29273 - t29280 - t32259 - 0.17090058289204942853e-2 * t1850 * t5396 * t32261 + t32266 + t32269 - t32272 + t32275 - t32277 - t32281 - t32285 - t32289;
    (t32290,)
}
