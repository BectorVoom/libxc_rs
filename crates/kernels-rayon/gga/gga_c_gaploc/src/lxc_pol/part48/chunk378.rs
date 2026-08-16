//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 378/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk378(t3225: f64, t928: f64, t295: f64, t3113: f64, t2558: f64, t954: f64, t943: f64, t2571: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t3226 = t3225 * t928;
    let t3232 = t295 * t3113;
    let t3240 = t954 * t2558;
    let t3242 = 0.64087718584518535698e-3_f64 * t943 * t3240;
    let t3247 = t883 * t2571;
    (t3226, t3232, t3240, t3242, t3247)
}
