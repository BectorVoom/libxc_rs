//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 987/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk987(t12218: f64, t2508: f64, t7226: f64, t7291: f64, t13937: f64, t2549: f64, t12176: f64, t2558: f64, t943: f64, t1841: f64, t47484: f64, t7289: f64) -> (f64, f64, f64, f64) {
    let t47685 = t2508 * t7226 * t12218 * t7291;
    let t47687 = t2549 * t13937;
    let t47690 = t943 * t12176 * t2558;
    let t47693 = t1841 * t7289 * t47484;
    (t47685, t47687, t47690, t47693)
}
