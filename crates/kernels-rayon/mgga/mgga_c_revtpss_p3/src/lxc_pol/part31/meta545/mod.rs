//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1933;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1934;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta545(t1949: f64, t231: f64, t6016: f64, t7076: f64, t1558: f64, t1579: f64, t25392: f64, t5977: f64, t2723: f64, t25416: f64, t1955: f64, t6041: f64, t1959: f64, t25333: f64, t25337: f64, t25362: f64, t25364: f64, t25371: f64, t25391: f64, t25406: f64, t25424: f64, t27199: f64, t27280: f64, t27325: f64, t27335: f64, t27338: f64, t27342: f64, t27344: f64, t7070: f64, t7775: f64, t29672: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29674, t29675, t29682, t29683, t29690, t29691, t29694, t29695, t29698) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1933(t1949, t231, t6016, t7076, t1558, t1579, t25392, t5977, t2723, t25416, t1955, t6041);
        let t29703 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1934(t1959, t25333, t25337, t25362, t25364, t25371, t25391, t25406, t25424, t27199, t27280, t27325, t27335, t27338, t27342, t27344, t29675, t29683, t29691, t29695, t29698, t7070, t7775);
        let (t29704, t29705) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1935(t29672, t29703, t892);
    (t29674, t29675, t29682, t29683, t29690, t29691, t29694, t29695, t29698, t29704, t29705)
}
