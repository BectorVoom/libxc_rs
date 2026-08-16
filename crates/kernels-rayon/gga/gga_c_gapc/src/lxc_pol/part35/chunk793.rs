//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 793/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk793(t9269: f64, t9272: f64, t1839: f64, t6: f64, t134: f64, t1509: f64, t2998: f64, t3004: f64, t1: f64, t1453: f64, t519: f64, t1030: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9273 = t9269 * t9272;
    let t9275 = t1839 * t6;
    let t9276 = t134 * t1509;
    let t9277 = t9275 * t9276;
    let t9278 = t2998 * t9277;
    let t9279 = t3004 * t9278;
    let t9281 = t1453 * t1;
    let t9282 = t519 * t9281;
    let t9283 = t1030 * t9282;
    (t9273, t9278, t9279, t9281, t9282, t9283)
}
