//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2541/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2541(t549: f64, t240: f64, t72: f64, t4014: f64, t9779: f64, t221: f64, t3978: f64, t3979: f64, t9628: f64, t1408: f64, t2237: f64, t2482: f64) -> (f64, f64, f64, f64) {
    let t46624 = t549 * t549;
    let t46625 = 1.0_f64 / t46624;
    let t46627 = t240 * t46625 * t72;
    let t46633 = t9779 * t4014;
    let t46641 = t3978 * t3979 * t221 * t9628;
    let t46644 = t2482 * t1408 * t2237;
    (t46627, t46633, t46641, t46644)
}
