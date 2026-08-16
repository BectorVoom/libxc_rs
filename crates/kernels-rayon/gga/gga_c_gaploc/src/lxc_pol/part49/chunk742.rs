//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 742/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk742(t3228: f64, t871: f64, t3113: f64, t931: f64, t12411: f64, t295: f64, t3276: f64, t7301: f64, t943: f64, t883: f64, t9603: f64, t7296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12573 = t3228 * t871;
    let t12574 = t931 * t3113;
    let t12580 = t295 * t12411;
    let t12604 = t3276 * t7301;
    let t12605 = t943 * t12604;
    let t12607 = t883 * t9603;
    let t12608 = t7296 * t12607;
    (t12573, t12574, t12580, t12604, t12605, t12608)
}
