//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 390/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk390(t169: f64, t3516: f64, t172: f64, t452: f64, t3094: f64, t3107: f64, t3099: f64, t3104: f64, t3114: f64, t3336: f64, t471: f64) -> (f64, f64, f64, f64, f64) {
    let t3517 = t3516 * t169;
    let t3518 = t3517 * t172;
    let t3519 = t452 * t3518;
    let t3522 = 3.0_f64 / 64.0_f64 * t3094;
    let t3525 = t3107 / 64.0_f64;
    let t3526 = t3522 - 9.0_f64 / 2048.0_f64 * t3099 + 3.0_f64 / 2048.0_f64 * t3104 - t3525;
    let t3529 = t3526 * t471 - 2.0_f64 * t3114 + t3336 + t3522 - t3525;
    (t3517, t3518, t3519, t3526, t3529)
}
