//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3142/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3142(t1214: f64, t22688: f64, t21107: f64, t5265: f64, t1247: f64, t24772: f64, t3172: f64, t20819: f64, t5292: f64, t17505: f64, t20783: f64, t1260: f64, t24699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82543 = t22688 * t1214;
    let t82550 = t21107 * t5265;
    let t82553 = t1247 * t3172 * t24772;
    let t82555 = t20819 * t5292;
    let t82560 = t17505 * t20783;
    let t82565 = t24699 * t1260;
    (t82543, t82550, t82553, t82555, t82560, t82565)
}
