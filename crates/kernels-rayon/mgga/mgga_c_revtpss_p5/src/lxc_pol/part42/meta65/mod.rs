//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk393;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk394;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk395;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk396;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk397;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta65(t1269: f64, t225: f64, t494: f64, t460: f64, t487: f64, t493: f64, t473: f64, t1214: f64, t1032: f64, t1243: f64, t355: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1271, t1274) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk393(t1269, t225, t494, t460, t487);
        let (t1275, t1276, t1277) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk394(t493, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk395(t473, t487);
        let (t1281, t1284) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk396(t1214, t1280, t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk397(t1284, t460);
        let t1287 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk398(t355, t471);
    (t1271, t1274, t1275, t1276, t1277, t1280, t1281, t1284, t1285, t1287)
}
