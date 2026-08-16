//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1790;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta503(t25207: f64, t29598: f64, t1468: f64, t1544: f64, t30: f64, t5962: f64, t25262: f64, t6024: f64, t25270: f64, t6037: f64, t5980: f64, t7038: f64, t25237: f64, t5989: f64, t5993: f64, t7045: f64, t5985: f64, t7025: f64, t6019: f64, t6030: f64, t1558: f64, t1579: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29599, t29602, t29606, t29616, t29618, t29620) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1790(t25207, t29598, t1468, t1544, t30, t5962, t25262, t6024, t25270, t6037, t5980, t7038);
        let (t29623, t29627, t29629, t29631, t29633, t29682) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1791(t25237, t5989, t5993, t7045, t5985, t7025, t6019, t7038, t6030, t1558, t1579, t231);
    (t29599, t29602, t29606, t29616, t29618, t29620, t29623, t29627, t29629, t29631, t29633, t29682)
}
