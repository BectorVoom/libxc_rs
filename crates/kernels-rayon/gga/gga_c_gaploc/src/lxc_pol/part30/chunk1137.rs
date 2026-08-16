//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1137/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1137(t28229: f64, t3192: f64, t574: f64, t1641: f64, t9421: f64, t18364: f64, t6710: f64, t9438: f64, t7014: f64, t9552: f64, t20843: f64, t2487: f64, t3177: f64) -> (f64, f64, f64, f64, f64) {
    let t30542 = t574 * t28229 * t3192;
    let t30546 = t1641 * t9421;
    let t30572 = t6710 * t9438 * t18364;
    let t30574 = t7014 * t9552;
    let t30575 = 0.1022478025437886658e1_f64 * t30574;
    let t30578 = 0.11928910296775344344e1_f64 * t2487 * t20843 * t3177;
    (t30542, t30546, t30572, t30575, t30578)
}
