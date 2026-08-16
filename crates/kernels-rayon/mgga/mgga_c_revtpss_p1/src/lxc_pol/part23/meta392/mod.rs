//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta392(t17307: f64, t480: f64, t16708: f64, t16710: f64, t16712: f64, t1256: f64, t5258: f64, t5262: f64, t1804: f64, t3655: f64, t1786: f64, t1260: f64, t12987: f64, t15687: f64, t3623: f64, t3782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17308, t17319, t17320, t17321, t17337, t17339, t17340, t17342, t17344) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1742(t17307, t480, t16708, t16710, t16712, t1256, t5258, t5262, t1804, t3655, t1786, t1260, t12987);
        let t17350 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1743(t15687, t3623);
        let t17351 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1744(t17350, t3782);
    (t17308, t17319, t17320, t17321, t17337, t17339, t17340, t17342, t17344, t17350, t17351)
}
