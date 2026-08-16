//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 377/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk377(t3094: f64, t3107: f64, t3099: f64, t3104: f64, t471: f64, t871: f64, t984: f64, t2321: f64, t999: f64, t882: f64, t2765: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3330 = 3.0_f64 / 128.0_f64 * t3094;
    let t3333 = t3107 / 128.0_f64;
    let t3334 = t3330 - 9.0_f64 / 4096.0_f64 * t3099 + 3.0_f64 / 4096.0_f64 * t3104 - t3333;
    let t3335 = t3334 * t471;
    let t3336 = t984 * t871;
    let t3344 = t999 * t2321;
    let t3345 = t882 * t3344;
    let t3346 = 0.11856252764865062333e-2_f64 * t3345;
    let t3347 = t2765 * t888;
    (t3330, t3333, t3334, t3335, t3336, t3344, t3345, t3346, t3347)
}
