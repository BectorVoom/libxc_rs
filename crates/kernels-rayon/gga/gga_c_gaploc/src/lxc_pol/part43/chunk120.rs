//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 120/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk120(t1: f64, t158: f64, t106: f64, t544: f64, t123: f64, t408: f64, t160: f64) -> (f64, f64, f64) {
    let t545 = t158 * t1;
    let t546 = t545 * t106;
    let t547 = t544 * t546;
    let t548 = t408 * t123;
    let t549 = t548 * t160;
    (t546, t547, t549)
}
