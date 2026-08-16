//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1694;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta442(t1774: f64, t3568: f64, t247: f64, t3719: f64, t15687: f64, t3623: f64, t3782: f64, t1263: f64, t1794: f64, t372: f64, t12712: f64, t3629: f64, t17301: f64, t17304: f64, t17308: f64, t17311: f64, t17333: f64, t17337: f64, t17339: f64, t17340: f64, t17342: f64, t17344: f64, t3674: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t17345, t17347, t17350, t17351, t17353, t17354) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1694(t1774, t3568, t247, t3719, t15687, t3623, t3782, t1263, t1794, t372, t12712, t3629);
        let (t17355, t17358) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1695(t17353, t17354, t17301, t17304, t17308, t17311, t17333, t17337, t17339, t17340, t17342, t17344, t17347, t17351, t3674, t484);
    (t17345, t17347, t17350, t17353, t17355, t17358)
}
