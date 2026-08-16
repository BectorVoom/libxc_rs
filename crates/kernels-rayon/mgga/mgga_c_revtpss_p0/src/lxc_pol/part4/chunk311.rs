//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 311/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk311(t344: f64, t614: f64, t139: f64, t221: f64, t346: f64, t345: f64, t220: f64) -> (f64, f64, f64, f64) {
    let t1003 = t614 * t344;
    let t1007 = t221 * t139 * t346;
    let t1009 = t345 * t1007 / 288.0_f64;
    let t1010 = t344 * t220;
    (t1003, t1007, t1009, t1010)
}
