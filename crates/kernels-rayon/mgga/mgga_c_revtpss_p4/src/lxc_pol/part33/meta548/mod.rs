//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta548(t1450: f64, t6816: f64, t7237: f64, t2014: f64, t6836: f64, t25864: f64, t1843: f64, t7741: f64, t651: f64, t196: f64, t197: f64, t6773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29494, t29495, t29497, t29498, t29499, t29501, t29502, t29504, t29506) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1932(t1450, t6816, t7237, t2014, t6836, t25864, t1843, t7741, t651, t196, t197, t6773);
    (t29494, t29495, t29497, t29498, t29499, t29501, t29502, t29504, t29506)
}
