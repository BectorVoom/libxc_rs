//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 805/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk805(t136: f64, t561: f64, t2457: f64, t3906: f64, t1420: f64, t786: f64, t1364: f64, t1426: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3907 = t561 * t136;
    let t3908 = t3907 * t2457;
    let t3910 = 0.11565819519348392139e-2_f64 * t3906 * t3908;
    let t3911 = t786 * t1420;
    let t3912 = t3911 * t1364;
    let t3914 = t556 * t1426;
    let t3915 = t786 * t3914;
    (t3907, t3908, t3910, t3911, t3912, t3914, t3915)
}
