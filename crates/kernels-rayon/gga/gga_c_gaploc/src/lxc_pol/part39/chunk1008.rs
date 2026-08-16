//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1008/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1008(t11004: f64, t2508: f64, t7226: f64, t7291: f64, t40902: f64, t10789: f64, t7667: f64, t13188: f64, t7137: f64, t13191: f64, t7129: f64, t24660: f64, t3251: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43298 = t2508 * t7226 * t11004 * t7291;
    let t43300 = 0.64087718584518535698e-3_f64 * t40902;
    let t43302 = t2508 * t10789 * t7667;
    let t43304 = t7137 * t13188;
    let t43312 = 0.92286314761706691403e-1_f64 * t7129 * t13191;
    let t43315 = 0.92286314761706691403e-1_f64 * t2508 * t24660 * t3251;
    (t43298, t43300, t43302, t43304, t43312, t43315)
}
