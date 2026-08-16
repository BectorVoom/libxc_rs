//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta289(t10981: f64, t10982: f64, t2455: f64, t9285: f64, t2454: f64, t252: f64, t2769: f64, t786: f64, t2435: f64, t2448: f64, t2440: f64, t887: f64, t2439: f64, t866: f64, t225: f64, t2461: f64, t2471: f64, t788: f64, t9288: f64, t787: f64, t2453: f64, t861: f64, t2458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10984, t10987, t10995, t11000, t11003) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1048(t10981, t10982, t2455, t9285, t2454, t252, t2769, t786, t2435, t2448, t2440, t887);
        let (t11004, t11008, t11013, t11017, t11019) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1049(t11003, t2439, t866, t225, t2461, t2471, t788, t9288, t787, t2453, t861, t2458);
    (t10984, t10987, t10995, t11000, t11004, t11008, t11013, t11017, t11019)
}
