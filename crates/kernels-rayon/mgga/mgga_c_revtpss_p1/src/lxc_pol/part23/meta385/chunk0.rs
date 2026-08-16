//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1730/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1730(t16710: f64, t16712: f64, t1130: f64, t5060: f64, t1719: f64, t3432: f64) -> (f64, f64, f64, f64) {
    let t16821 = 0.12361111111111111111e-1_f64 * t16710;
    let t16822 = 0.61805555555555555556e-2_f64 * t16712;
    let t16835 = t5060 * t1130;
    let t16840 = t1719 * t3432;
    (t16821, t16822, t16835, t16840)
}
