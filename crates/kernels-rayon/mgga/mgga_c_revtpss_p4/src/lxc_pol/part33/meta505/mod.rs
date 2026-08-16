//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta505(t1937: f64, t27123: f64, t4292: f64, t94: f64, t6993: f64, t7732: f64, t7003: f64, t2322: f64, t7735: f64, t4254: f64, t1936: f64, t5517: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27125, t27126, t27128, t27130, t27132, t27134, t27136, t27137) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1824(t1937, t27123, t4292, t94, t6993, t7732, t7003, t2322, t7735, t4254, t1936, t5517);
    (t27125, t27126, t27128, t27130, t27132, t27134, t27136, t27137)
}
