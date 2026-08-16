//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 382/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk382(t169: f64, t3338: f64, t172: f64, t452: f64, t2321: f64, t999: f64, t882: f64, t2765: f64, t888: f64, t2268: f64, t2778: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3339 = t3338 * t169;
    let t3340 = t3339 * t172;
    let t3341 = t452 * t3340;
    let t3344 = t999 * t2321;
    let t3345 = t882 * t3344;
    let t3346 = 0.11856252764865062333e-2_f64 * t3345;
    let t3347 = t2765 * t888;
    let t3349 = 0.85365019907028448797e-1_f64 * t2268 * t3347;
    let t3350 = t883 * t2778;
    (t3339, t3340, t3341, t3344, t3345, t3346, t3347, t3349, t3350)
}
