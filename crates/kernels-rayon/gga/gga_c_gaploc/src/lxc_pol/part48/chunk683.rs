//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 683/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk683(t13176: f64, t943: f64, t10924: f64, t2558: f64, t9647: f64, t10628: f64, t5539: f64, t10697: f64, t3247: f64, t10677: f64, t883: f64, t2562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13177 = t943 * t13176;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13224 = t883 * t10677;
    let t13225 = t2562 * t13224;
    (t13177, t13182, t13183, t13194, t13195, t13200, t13201, t13225)
}
