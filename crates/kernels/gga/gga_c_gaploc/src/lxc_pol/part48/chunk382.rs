//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 382/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk382<F: Float>(t169: F, t3338: F, t172: F, t452: F, t2321: F, t999: F, t882: F, t2765: F, t888: F, t2268: F, t2778: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3339 = t3338 * t169;
    let t3340 = t3339 * t172;
    let t3341 = t452 * t3340;
    let t3344 = t999 * t2321;
    let t3345 = t882 * t3344;
    let t3346 = F::cast_from(0.11856252764865062333e-2_f64) * t3345;
    let t3347 = t2765 * t888;
    let t3349 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t3347;
    let t3350 = t883 * t2778;
    (t3339, t3340, t3341, t3344, t3345, t3346, t3347, t3349, t3350)
}
