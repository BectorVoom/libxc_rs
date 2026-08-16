//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 479/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk479(t1: f64, t2293: f64, t106: f64, t192: f64, t1406: f64, t2486: f64) -> (f64, f64) {
    let t7005 = t2293 * t1;
    let t7006 = t7005 * t106;
    let t7007 = t7006 * t192;
    let t7014 = t1406 * t2486;
    (t7007, t7014)
}
