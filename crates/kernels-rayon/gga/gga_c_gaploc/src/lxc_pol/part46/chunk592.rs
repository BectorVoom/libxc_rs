//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 592/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk592(t10148: f64, t10181: f64, t10230: f64, t10279: f64, t3362: f64, t501: f64, t3366: f64, t605: f64, t2902: f64, t921: f64, t1016: f64, t2497: f64) -> (f64, f64, f64, f64, f64) {
    let t10281 = t10148 + t10181 + t10230 + t10279;
    let t10283 = t3362 * t501;
    let t10295 = t3366 * t605;
    let t10298 = t2902 * t921;
    let t10301 = t1016 * t2497;
    (t10281, t10283, t10295, t10298, t10301)
}
