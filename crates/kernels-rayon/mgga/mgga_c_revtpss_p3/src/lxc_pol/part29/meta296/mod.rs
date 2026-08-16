//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta296(t1376: f64, t9789: f64, t235: f64, t4086: f64, t2453: f64, t240: f64, t2712: f64, t3994: f64, t2713: f64, t3951: f64, t3964: f64, t785: f64, t9731: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9791, t9793, t9794, t9795, t9796, t9799, t9801) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1184(t1376, t9789, t235, t4086, t2453, t240, t2712, t3994, t2713, t3951, t3964, t785, t9731);
    (t9791, t9793, t9794, t9795, t9796, t9799, t9801)
}
