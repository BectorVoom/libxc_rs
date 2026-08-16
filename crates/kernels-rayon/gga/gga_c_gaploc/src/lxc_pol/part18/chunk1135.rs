//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1135/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1135(t30292: f64, t9287: f64, t1305: f64, t2476: f64, t9438: f64, t9439: f64, t6974: f64, t9441: f64, t7014: f64, t9450: f64, t1411: f64, t3177: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t30294 = 0.29792074959875355558e-1_f64 * t30292 * t9287;
    let t30297 = t2476 * t9438 * t9439 * t1305;
    let t30299 = t6974 * t9441;
    let t30305 = t7014 * t9450;
    let t30323 = 0.11928910296775344344e1_f64 * t587 * t1411 * t3177;
    (t30294, t30297, t30299, t30305, t30323)
}
