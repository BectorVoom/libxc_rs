//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1718/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1718(t3995: f64, t40488: f64, t3989: f64, t9944: f64, t549: f64, t240: f64, t72: f64, t3829: f64, t4014: f64, t9779: f64, t221: f64, t3978: f64, t3979: f64, t9628: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46620 = t40488 * t3995;
    let t46622 = t3989 * t9944;
    let t46624 = t549 * t549;
    let t46625 = 1.0_f64 / t46624;
    let t46627 = t240 * t46625 * t72;
    let t46628 = t3829 * t3829;
    let t46633 = t9779 * t4014;
    let t46641 = t3978 * t3979 * t221 * t9628;
    (t46620, t46622, t46627, t46628, t46633, t46641)
}
