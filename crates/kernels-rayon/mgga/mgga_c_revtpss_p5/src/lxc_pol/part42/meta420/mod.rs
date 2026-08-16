//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1481;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta420(t5891: f64, t8311: f64, t1513: f64, t31429: f64, t1509: f64, t8315: f64, t5915: f64, t109: f64, t1479: f64, t655: f64, t31433: f64, t31149: f64, t5907: f64, t5911: f64, t31035: f64, t31134: f64, t31415: f64, t31427: f64, t31437: f64, t69: f64, t8258: f64, t8267: f64, t114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31626, t31629, t31633, t31636, t31640, t31643, t31646) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1480(t5891, t8311, t1513, t31429, t1509, t8315, t5915, t109, t1479, t655, t31433, t31149, t5907);
        let (t31649, t31652) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1481(t5911, t8315, t31035, t31134, t31415, t31427, t31437, t31626, t31629, t31633, t31636, t31640, t31643, t31646, t69, t8258, t8267);
        let t31653 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1482(t114, t31652);
    (t31626, t31629, t31633, t31636, t31640, t31643, t31646, t31649, t31653)
}
