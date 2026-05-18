//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 402/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk402<F: Float>(t894: F, t988: F, t2268: F, t3094: F, t3107: F, t3099: F, t3104: F, t471: F, t2321: F, t999: F, t882: F, t2765: F, t888: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3327 = t894 * t988;
    let t3329 = F::new(0.28455006635676149599e-1) * t2268 * t3327;
    let t3330 = F::new(3.0) / F::new(128.0) * t3094;
    let t3333 = t3107 / F::new(128.0);
    let t3334 = t3330 - F::new(9.0) / F::new(4096.0) * t3099 + F::new(3.0) / F::new(4096.0) * t3104 - t3333;
    let t3335 = t3334 * t471;
    let t3344 = t999 * t2321;
    let t3345 = t882 * t3344;
    let t3346 = F::new(0.11856252764865062333e-2) * t3345;
    let t3347 = t2765 * t888;
    (t3327, t3329, t3330, t3333, t3334, t3335, t3344, t3346, t3347)
}
