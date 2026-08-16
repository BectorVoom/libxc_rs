//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1179;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1180;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1181;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta299(t1120: f64, t12273: f64, t128: f64, t12287: f64, t12277: f64, t12292: f64, t12296: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t1132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12313, t12314) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1179(t1120, t12273, t128);
        let (t12316, t12317) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1180(t1120, t12287, t128);
        let (t12319, t12320) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1181(t1120, t12277, t128);
        let (t12322, t12323) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1182(t12292, t12296, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t1132);
    (t12313, t12314, t12316, t12317, t12319, t12320, t12322, t12323)
}
