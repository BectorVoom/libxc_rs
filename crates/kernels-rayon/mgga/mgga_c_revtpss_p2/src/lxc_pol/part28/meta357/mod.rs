//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1378;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1379;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1380;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta357(t3431: f64, t418: f64, t408: f64, t3418: f64, t698: f64, t240: f64, t3698: f64, t3361: f64, t635: f64, t1146: f64, t2439: f64, t3424: f64, t3421: f64, t57: f64, t268: f64, t404: f64, t7021: f64, t1123: f64, t2435: f64, t3364: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12248, t12252, t12254, t12256, t12261, t12263) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1378(t3431, t418, t408, t3418, t698, t240, t3698, t3361, t635, t1146, t2439, t3424);
        let (t12265, t12268, t12295) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1379(t3421, t698, t3361, t57, t268, t404, t7021);
        let (t12296, t12297) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1380(t12295, t1123, t2435);
        let t12299 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1381(t3364, t689);
    (t12248, t12252, t12254, t12256, t12261, t12263, t12265, t12268, t12295, t12296, t12297, t12299)
}
