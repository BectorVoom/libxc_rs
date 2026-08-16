//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2435/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2435(t46456: f64, t786: f64, t1398: f64, t268: f64, t4101: f64, t543: f64, t793: f64, t544: f64, t9989: f64, t4003: f64, t10013: f64, t2453: f64) -> (f64, f64, f64, f64, f64) {
    let t46457 = t786 * t46456;
    let t46463 = t4101 * t268 * t793 * t1398 * t543;
    let t46475 = 1.0_f64 / t9989 / t544;
    let t46478 = t4003 * t4003;
    let t46495 = t2453 * t10013;
    (t46457, t46463, t46475, t46478, t46495)
}
