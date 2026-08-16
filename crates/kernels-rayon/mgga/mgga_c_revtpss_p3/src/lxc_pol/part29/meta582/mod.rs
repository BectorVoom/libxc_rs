//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1933;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta582(t11064: f64, t1113: f64, t27384: f64, t27799: f64, t98767: f64, t33: f64, t41154: f64, t98786: f64, t1711: f64, t2411: f64, t14365: f64, t1544: f64, t3351: f64, t4343: f64, t1583: f64, t63164: f64, t4433: f64, t892: f64, t14749: f64, t27763: f64, t14767: f64, t2408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100975, t100978, t100982, t100988, t100993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1933(t11064, t1113, t27384, t27799, t98767, t33, t41154, t98786, t1711, t2411, t14365, t1544, t3351);
        let (t100997, t101012, t101016, t101029, t101032, t101035, t101040) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1934(t1113, t4343, t1583, t3351, t27799, t63164, t4433, t892, t14749, t27763, t14767, t1711, t2408);
    (t100975, t100978, t100982, t100988, t100993, t100997, t101012, t101016, t101029, t101032, t101035, t101040)
}
