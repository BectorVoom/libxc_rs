//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1702/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1702(t3911: f64, t3920: f64, t3957: f64, t3961: f64, t3829: f64, t4011: f64, t547: f64, t807: f64, t2237: f64, t240: f64, t550: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9695 = t3911 * t3920;
    let t9697 = t3957 * t3961;
    let t9703 = t4011 * t3829;
    let t9704 = t547 * t9703;
    let t9705 = t807 * t9704;
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    (t9695, t9697, t9703, t9704, t9705, t9707, t9709)
}
