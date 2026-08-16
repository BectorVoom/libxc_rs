//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2941/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2941(t10026: f64, t48084: f64, t136: f64, t2457: f64, t3964: f64, t5710: f64, t221: f64, t9817: f64, t13792: f64, t13845: f64, t1882: f64, t9994: f64) -> (f64, f64, f64, f64, f64) {
    let t48085 = t48084 * t10026;
    let t48089 = t3964 * t5710 * t136 * t2457;
    let t48100 = t9817 * t221;
    let t48102 = t13845 * t48100 * t13792;
    let t48105 = t1882 * t9994;
    (t48085, t48089, t48100, t48102, t48105)
}
