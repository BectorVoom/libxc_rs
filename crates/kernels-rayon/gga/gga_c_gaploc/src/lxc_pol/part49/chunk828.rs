//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 828/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk828(t501: f64, t9241: f64, t5538: f64, t883: f64, t28668: f64, t7290: f64, t2547: f64, t279: f64, t481: f64, t747: f64, t9765: f64, t1959: f64, t3259: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29096 = t9241 * t501;
    let t29277 = t5538 * t883;
    let t29285 = t7290 * t28668;
    let t29439 = t481 * t2547 * t279;
    let t29646 = t9765 * t747;
    let t29650 = t3259 * t1959;
    (t29096, t29277, t29285, t29439, t29646, t29650)
}
