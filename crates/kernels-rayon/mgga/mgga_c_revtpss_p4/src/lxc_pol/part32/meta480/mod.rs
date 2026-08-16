//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1719;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1720;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta480(t25207: f64, t27375: f64, t11064: f64, t30: f64, t1583: f64, t890: f64, t605: f64, t4537: f64, t1468: f64, t775: f64, t33: f64, t892: f64, t4433: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27376, t27383) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1719(t25207, t27375, t11064, t30);
        let t27384 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1720(t1583, t890);
        let (t27385, t27387, t27391, t27395, t27402, t27763, t27764) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1721(t27383, t27384, t1583, t605, t30, t4537, t1468, t775, t890, t33, t892, t4433);
    (t27376, t27383, t27384, t27385, t27387, t27391, t27395, t27402, t27763, t27764)
}
