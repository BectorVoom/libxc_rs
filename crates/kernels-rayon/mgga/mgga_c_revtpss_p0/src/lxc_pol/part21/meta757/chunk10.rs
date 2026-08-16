//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2665/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2665(t13821: f64, t13999: f64, t13716: f64, t1413: f64, t547: f64, t807: f64, t550: f64, t9794: f64, t14224: f64, t9793: f64, t13928: f64, t9962: f64) -> (f64, f64, f64, f64, f64) {
    let t49062 = t13999 * t13821;
    let t49066 = t807 * t547 * t1413 * t13716;
    let t49068 = t9794 * t550;
    let t49070 = t9793 * t49068 * t14224;
    let t49071 = 0.13553694749236397037e-4_f64 * t49070;
    let t49085 = t9962 * t13928;
    (t49062, t49066, t49068, t49071, t49085)
}
