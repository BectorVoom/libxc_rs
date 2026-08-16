//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 548/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk548(t45: f64, t57: f64, t1522: f64, t706: f64, t1469: f64, t78: f64, t81: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1524 = 4.0_f64 * t706 * t1522;
    let t1527 = piecewise3(t151, 0.0_f64, 4.0_f64 / 3.0_f64 * t78 * t1469);
    let t1530 = piecewise3(t155, 0.0_f64, -4.0_f64 / 3.0_f64 * t81 * t1469);
    let t1531 = t1527 + t1530;
    (t1524, t1531)
}
