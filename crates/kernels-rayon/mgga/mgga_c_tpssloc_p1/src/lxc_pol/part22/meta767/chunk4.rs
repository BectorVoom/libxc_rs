//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2596/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2596(t1009: f64, t22113: f64, t1011: f64, t1212: f64, t18375: f64, t5002: f64, t1218: f64, t1737: f64, t18943: f64, t19080: f64, t5014: f64, t65617: f64, t65619: f64, t65628: f64, t65632: f64, t65637: f64, t65647: f64, t65649: f64, t65651: f64, t66159: f64) -> (f64, f64) {
    let t72361 = t22113 * t1009;
    let t72363 = t72361 * t1011 * t1212;
    let t72366 = t5002 * t18375;
    let t72380 = -t65617 / 2304.0_f64 - t65619 / 2304.0_f64 + t72363 * t1218 / 3072.0_f64 + t72366 / 1536.0_f64 - t65628 / 648.0_f64 + t65632 / 4608.0_f64 + t65637 / 27.0_f64 + t5002 * t18943 / 1024.0_f64 + t65647 / 6912.0_f64 - t66159 * t1737 / 96.0_f64 - t19080 * t5014 / 96.0_f64 - t65649 / 2304.0_f64 - t65651 / 144.0_f64;
    (t72361, t72380)
}
