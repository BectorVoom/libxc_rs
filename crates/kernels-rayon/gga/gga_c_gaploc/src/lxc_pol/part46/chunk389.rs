//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 389/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk389(t169: f64, t3234: f64, t299: f64, t706: f64, t2558: f64, t954: f64, t943: f64, t3210: f64, t325: f64, t738: f64, t2571: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3235 = t3234 * t169;
    let t3236 = t3235 * t299;
    let t3237 = t706 * t3236;
    let t3240 = t954 * t2558;
    let t3242 = 0.64087718584518535698e-3_f64 * t943 * t3240;
    let t3243 = t3210 * t325;
    let t3244 = t738 * t3243;
    let t3247 = t883 * t2571;
    (t3236, t3237, t3240, t3242, t3243, t3244, t3247)
}
