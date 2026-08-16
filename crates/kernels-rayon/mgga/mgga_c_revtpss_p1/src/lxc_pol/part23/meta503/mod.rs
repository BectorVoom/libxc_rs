//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1991;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1992;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta503(t20849: f64, t225: f64, t480: f64, t1238: f64, t17296: f64, t17298: f64, t17301: f64, t17304: f64, t17337: f64, t17609: f64, t1797: f64, t20838: f64, t20843: f64, t20847: f64, t5274: f64, t5287: f64, t5293: f64, t5331: f64) -> (f64, f64, f64) {
        let t20850 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1991(t20849, t225);
        let t20851 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1992(t20850, t480);
        let t20855 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1993(t1238, t17296, t17298, t17301, t17304, t17337, t17609, t1797, t20838, t20843, t20847, t20851, t5274, t5287, t5293, t5331);
    (t20850, t20851, t20855)
}
