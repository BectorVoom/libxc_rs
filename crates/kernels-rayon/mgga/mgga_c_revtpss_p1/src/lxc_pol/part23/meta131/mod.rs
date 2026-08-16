//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk853;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk854;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta131(t1235: f64, t3678: f64, t221: f64, t462: f64, t696: f64, t461: f64, t1226: f64, t140: f64, t1222: f64, t1224: f64, t3367: f64, t1121: f64, t404: f64, t3362: f64, t1251: f64, t3172: f64, t1247: f64, t1032: f64, t1204: f64, t1246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3679, t3682, t3684, t3685, t3686, t3692, t3698) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk853(t1235, t3678, t221, t462, t696, t461, t1226, t140, t1222, t1224, t3367, t1121, t404);
        let (t3699, t3704) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk854(t3362, t3698, t1251, t3172);
        let (t3705, t3707, t3708) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk855(t1247, t3704, t1032, t1204, t1246);
    (t3679, t3682, t3684, t3685, t3686, t3692, t3698, t3699, t3704, t3705, t3707, t3708)
}
