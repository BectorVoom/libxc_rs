//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1722;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta481(t18875: f64, t25759: f64, t1113: f64, t1544: f64, t33: f64, t4343: f64, t27375: f64, t11064: f64, t27384: f64, t1583: f64, t4537: f64, t1711: f64, t775: f64, t890: f64, t196: f64, t197: f64, t5528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27770, t27773, t27777, t27793, t27799) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1722(t18875, t25759, t1113, t1544, t33, t4343, t27375, t11064);
        let (t27800, t27802, t27806, t27810, t27817, t27833) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1723(t27384, t27799, t1113, t1583, t33, t4537, t1711, t775, t890, t196, t197, t5528);
    (t27770, t27773, t27777, t27793, t27799, t27800, t27802, t27806, t27810, t27817, t27833)
}
