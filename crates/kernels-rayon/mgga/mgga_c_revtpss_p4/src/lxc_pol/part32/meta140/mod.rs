//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk716;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk717;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk718;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta140(t1222: f64, t3685: f64, t1224: f64, t3367: f64, t1121: f64, t404: f64, t3362: f64, t1251: f64, t3172: f64, t1247: f64, t1032: f64, t1204: f64, t1246: f64, t1234: f64, t1260: f64, t1209: f64, t1284: f64, t3624: f64, t482: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3686, t3692, t3698, t3699, t3704, t3705, t3707) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk716(t1222, t3685, t1224, t3367, t1121, t404, t3362, t1251, t3172, t1247, t1032, t1204);
        let (t3708, t3711) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk717(t1246, t3707, t1234, t1260);
        let (t3717, t3718) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk718(t1209, t1284, t3624);
        let t3719 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk719(t482, t66);
    (t3686, t3692, t3698, t3699, t3704, t3705, t3707, t3708, t3711, t3717, t3718, t3719)
}
