//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1483;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1484;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta406(t4287: f64, t8311: f64, t625: f64, t8399: f64, t109: f64, t55: f64, t665: f64, t108: f64, t661: f64, t31032: f64, t8402: f64, t1509: f64, t8315: f64, t31149: f64, t2: f64, t31035: f64, t31134: f64, t31135: f64, t31137: f64, t31287: f64, t31415: f64, t31417: f64, t31421: f64, t8258: f64, t8267: f64, t114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31424, t31427, t31429, t31430, t31433, t31434, t31437, t31439) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1483(t4287, t8311, t625, t8399, t109, t55, t665, t108, t661, t31032, t8402, t1509);
        let (t31440, t31444, t31447, t31450) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1484(t31439, t8315, t1509, t661, t31149, t2, t31035, t31134, t31135, t31137, t31287, t31415, t31417, t31421, t31424, t31427, t31430, t31434, t31437, t8258, t8267);
        let t31451 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1485(t114, t31450);
    (t31424, t31429, t31430, t31433, t31434, t31440, t31444, t31447, t31451)
}
