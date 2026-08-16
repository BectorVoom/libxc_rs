//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 809/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk809(t13200: f64, t1841: f64, t13182: f64, t29439: f64, t11083: f64, t2558: f64, t943: f64, t13225: f64, t731: f64, t13176: f64, t2549: f64, t33232: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43098 = t1841 * t13200;
    let t43100 = t29439 * t13182;
    let t43127 = t943 * t11083 * t2558;
    let t43139 = t731 * t13225;
    let t43196 = t2549 * t13176;
    let t43224 = t9647 * t33232 * t2558;
    (t43098, t43100, t43127, t43139, t43196, t43224)
}
