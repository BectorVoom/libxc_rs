//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 401/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk401(t169: f64, t3601: f64, t299: f64, t706: f64, t3216: f64, t3226: f64, t3218: f64, t3223: f64, t3232: f64, t3429: f64, t471: f64) -> (f64, f64, f64, f64, f64) {
    let t3602 = t3601 * t169;
    let t3603 = t3602 * t299;
    let t3604 = t706 * t3603;
    let t3607 = 3.0_f64 / 64.0_f64 * t3216;
    let t3610 = t3226 / 64.0_f64;
    let t3611 = t3607 - 9.0_f64 / 2048.0_f64 * t3218 + 3.0_f64 / 2048.0_f64 * t3223 - t3610;
    let t3614 = t3611 * t471 - 2.0_f64 * t3232 + t3429 + t3607 - t3610;
    (t3602, t3603, t3604, t3611, t3614)
}
