//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1816;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1817;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta482(t25875: f64, t25898: f64, t1399: f64, t676: f64, t25880: f64, t25894: f64) -> (f64, f64, f64, f64, f64) {
        let t25899 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1816(t25875, t25898);
        let (t25900, t25901) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1817(t1399, t676, t25880);
        let (t25902, t25904) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1818(t25899, t25901, t25894, t25898);
    (t25899, t25900, t25901, t25902, t25904)
}
