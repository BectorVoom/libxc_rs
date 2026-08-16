//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1936;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1937;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta546(t30: f64, t6079: f64, t1468: f64, t1583: f64, t6075: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25445: f64, t27368: f64, t29592: f64, t29599: f64, t29602: f64, t29606: f64, t29705: f64, t4541: f64, t5824: f64, t7091: f64, t7749: f64, t7783: f64, t7787: f64, t1651: f64, t7810: f64, t7145: f64, t1976: f64, t6392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t29713, t29716, t29719, t29726) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1936(t30, t6079, t1468, t1583, t6075, t1940, t1963, t2403, t25206, t25445, t27368, t29592, t29599, t29602, t29606, t29705, t4541, t5824, t7091, t7749, t7783, t7787);
        let t29727 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1937(t1651, t7810);
        let (t29728, t29731) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1938(t29727, t7145, t1976, t6392);
    (t29713, t29716, t29719, t29726, t29727, t29728, t29731)
}
