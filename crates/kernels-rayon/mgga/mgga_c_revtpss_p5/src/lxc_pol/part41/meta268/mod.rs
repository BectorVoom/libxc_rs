//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1015;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta268(t1376: f64, t9789: f64, t235: f64, t4086: f64, t2453: f64, t240: f64, t2712: f64, t3994: f64, t2713: f64, t3951: f64, t3964: f64, t785: f64, t9731: f64, t225: f64, t4062: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64, t220: f64, t1353: f64, t4003: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9791, t9793, t9794, t9796, t9799, t9801) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1015(t1376, t9789, t235, t4086, t2453, t240, t2712, t3994, t2713, t3951, t3964, t785, t9731);
        let (t9802, t9804, t9816, t9818, t9835, t9845) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1016(t225, t9801, t4062, t1386, t2482, t814, t136, t1412, t220, t1353, t4003, t2735, t4086);
    (t9791, t9793, t9794, t9796, t9799, t9802, t9804, t9816, t9818, t9835, t9845)
}
