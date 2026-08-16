//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1387/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1387(t794: f64, t9747: f64, t2699: f64, t3943: f64, t136: f64, t9941: f64, t1386: f64, t820: f64, t9948: f64, t159: f64, t216: f64, t4010: f64) -> (f64, f64, f64, f64, f64) {
    let t46691 = t794 * t9747;
    let t46694 = t2699 * t3943;
    let t46716 = t9941 * t136;
    let t46722 = t820 * t1386 * t9948;
    let t46730 = t216 * t159 * t4010;
    (t46691, t46694, t46716, t46722, t46730)
}
